use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]
use crate::action::Action;
use crate::actions::choose_dual_wield::can_dual_wield;
use crate::actions::damage::{DamageAction, DamageType};
use crate::actions::decrease_max_hp::DecreaseMaxHPAction;
use crate::actions::discard_card::DiscardCardAction;
use crate::actions::discovery::DiscoveryAction;
use crate::actions::draw::DrawAction;
use crate::actions::dual_wield::DualWieldAction;
use crate::actions::end_of_turn_discard::EndOfTurnDiscardAction;
use crate::actions::exhaust_card::ExhaustCardAction;
use crate::actions::forethought::ForethoughtAction;
use crate::actions::gain_status::GainStatusAction;
use crate::actions::memories::MemoriesAction;
use crate::actions::place_card_in_hand::PlaceCardInHandAction;
use crate::actions::place_card_on_top_of_draw::PlaceCardOnTopOfDrawAction;
use crate::actions::play_card::PlayCardAction;
use crate::actions::reduce_status::ReduceStatusAction;
use crate::actions::start_of_turn_energy::StartOfTurnEnergyAction;
use crate::actions::upgrade::UpgradeAction;
use crate::blessings::Blessing;
use crate::card::{Card, CardPile, CardRef};
use crate::cards::{CardClass, CardCost, CardType, transformed};
use crate::creature::Creature;
use crate::map::Map;
use crate::monster::{Monster, MonsterBehavior, MonsterInfo};
use crate::monsters::test::NoopMonster;
use crate::potion::Potion;
use crate::queue::ActionQueue;
use crate::relic::{Relic, RelicClass, new_relic};
use crate::rng::rand_slice;
use crate::state::{GameState, GameStateManager};
use crate::status::Status;
use crate::{assert_matches, assert_not_matches};
use rand::seq::SliceRandom;

pub type Rand = rand::rngs::ThreadRng;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CreatureRef(usize);

impl CreatureRef {
    pub fn player() -> Self {
        Self(0)
    }
    pub fn monster(n: usize) -> Self {
        Self(n + 1)
    }
    pub fn is_player(&self) -> bool {
        self.0 == 0
    }
    pub fn monster_index(&self) -> usize {
        assert!(!self.is_player());
        self.0 - 1
    }
}

impl std::fmt::Debug for CreatureRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_player() {
            write!(f, "player")
        } else {
            write!(f, "monster {}", self.0 - 1)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    ChooseBlessing(Blessing),
    Transform {
        card_index: usize,
    },
    EndTurn,
    PlayCard {
        card_index: usize,
        target: Option<usize>,
    },
    Armaments {
        card_index: usize,
    },
    PlaceCardInHandOnTopOfDraw {
        card_index: usize,
    },
    PlaceCardInDiscardOnTopOfDraw {
        card_index: usize,
    },
    Memories {
        card_index: usize,
    },
    ExhaustOneCardInHand {
        card_index: usize,
    },
    ExhaustCardsInHand {
        card_index: usize,
    },
    ExhaustCardsInHandEnd,
    Gamble {
        card_index: usize,
    },
    GambleEnd,
    DualWield {
        card_index: usize,
    },
    Exhume {
        card_index: usize,
    },
    FetchCardFromDraw {
        card_index: usize,
    },
    ForethoughtAny {
        card_index: usize,
    },
    ForethoughtAnyEnd,
    ForethoughtOne {
        card_index: usize,
    },
    Discovery {
        card_class: CardClass,
    },
    DiscardPotion {
        potion_index: usize,
    },
    UsePotion {
        potion_index: usize,
        target: Option<usize>,
    },
}

#[derive(Debug)]
pub enum GameStatus {
    Defeat,
    Victory,
    Combat,
    ExhaustCardsInHand { num_cards_remaining: i32 },
    Memories { num_cards_remaining: i32 },
}

#[derive(Default)]
#[allow(unused)]
pub struct GameBuilder {
    master_deck: Vec<(CardClass, bool)>,
    monsters: Vec<Monster>,
    monster_statuses: HashMap<Status, i32>,
    player_statuses: HashMap<Status, i32>,
    relics: Vec<RelicClass>,
    player_hp: Option<i32>,
    rng: Rand,
}

impl GameBuilder {
    pub fn ironclad_starting_deck(mut self) -> Self {
        for _ in 0..5 {
            self.master_deck.push((CardClass::Strike, false));
        }
        for _ in 0..4 {
            self.master_deck.push((CardClass::Defend, false));
        }
        self.master_deck.push((CardClass::Bash, false));
        self.master_deck.push((CardClass::AscendersBane, false));
        self
    }
    pub fn add_card(mut self, c: CardClass) -> Self {
        self.master_deck.push((c, false));
        self
    }
    pub fn add_card_upgraded(mut self, c: CardClass) -> Self {
        self.master_deck.push((c, true));
        self
    }
    #[cfg(test)]
    pub fn add_cards(mut self, c: CardClass, amount: i32) -> Self {
        for _ in 0..amount {
            self.master_deck.push((c, false));
        }
        self
    }
    #[cfg(test)]
    pub fn add_cards_upgraded(mut self, c: CardClass, amount: i32) -> Self {
        for _ in 0..amount {
            self.master_deck.push((c, true));
        }
        self
    }
    pub fn add_monster<M: MonsterBehavior + 'static>(mut self, m: M) -> Self {
        let hp = m.roll_hp(&mut self.rng);
        let name = m.name();

        self.monsters.push(Monster {
            creature: Creature::new(name, hp),
            behavior: Box::new(m),
        });
        self
    }

    #[cfg(test)]
    pub fn add_monster_status(mut self, s: Status, amount: i32) -> Self {
        self.monster_statuses.insert(s, amount);
        self
    }

    #[cfg(test)]
    pub fn add_player_status(mut self, s: Status, amount: i32) -> Self {
        self.player_statuses.insert(s, amount);
        self
    }
    pub fn add_relic(mut self, relic: RelicClass) -> Self {
        self.relics.push(relic);
        self
    }
    #[cfg(test)]
    pub fn set_player_hp(mut self, amount: i32) -> Self {
        self.player_hp = Some(amount);
        self
    }
    #[cfg(test)]
    pub fn build_combat(self) -> Game {
        use crate::state::GameState;

        let monster_statuses = self.monster_statuses.clone();
        let mut g = self.build();
        g.state.set_state(GameState::RollCombat);
        g.run();
        for m in &mut g.monsters {
            for (&k, &v) in &monster_statuses {
                m.creature.set_status(k, v);
            }
        }
        g
    }
    pub fn build(mut self) -> Game {
        if self.monsters.is_empty() {
            self = self.add_monster(NoopMonster::new());
        }
        let mut g = Game::new(self.rng, &self.master_deck, self.monsters);
        for (&k, &v) in &self.player_statuses {
            g.player.set_status(k, v);
        }
        for r in self.relics {
            g.add_relic(r);
        }
        if let Some(hp) = self.player_hp {
            g.player.cur_hp = hp;
        }
        g
    }
}

macro_rules! trigger {
    ($func_name:ident, $name:ident) => {
        pub fn $func_name(&mut self) {
            for r in &mut self.relics {
                r.$name(&mut self.action_queue);
            }
        }
    };
}

macro_rules! trigger_card {
    ($func_name:ident, $name:ident) => {
        pub fn $func_name(&mut self, play: &PlayCardAction) {
            for r in &mut self.relics {
                r.$name(&mut self.action_queue, play);
            }
        }
    };
}

pub struct Game {
    pub map: Map,
    pub player: Creature,
    pub relics: Vec<Relic>,
    pub potions: Vec<Option<Potion>>,
    pub gold: i32,
    pub monsters: Vec<Monster>,
    pub master_deck: CardPile,
    pub turn: i32,
    pub energy: i32,
    pub draw_per_turn: i32,
    pub draw_pile: CardPile,
    pub hand: CardPile,
    pub discard_pile: CardPile,
    pub exhaust_pile: CardPile,
    pub cur_card: Option<CardRef>,
    pub action_queue: ActionQueue,
    pub card_queue: Vec<PlayCardAction>,
    pub num_cards_played_this_turn: i32,
    pub combat_monsters_queue: Vec<Vec<Monster>>,
    pub rng: Rand,
    pub state: GameStateManager,
    next_id: u32,
}

impl Game {
    pub const MAX_HAND_SIZE: i32 = 10;

    fn new(mut rng: Rand, master_deck: &[(CardClass, bool)], monsters: Vec<Monster>) -> Self {
        let map = Map::generate(&mut rng);
        let mut g = Self {
            map,
            player: Creature::new("Ironclad", 80),
            relics: Default::default(),
            monsters: Default::default(),
            potions: vec![None; 2],
            gold: 0,
            master_deck: Default::default(),
            turn: 0,
            energy: 0,
            draw_per_turn: 5,
            draw_pile: Default::default(),
            hand: Default::default(),
            discard_pile: Default::default(),
            exhaust_pile: Default::default(),
            cur_card: None,
            action_queue: Default::default(),
            card_queue: Default::default(),
            num_cards_played_this_turn: 0,
            combat_monsters_queue: vec![monsters],
            rng,
            state: GameStateManager::new(GameState::Blessing),
            next_id: 1,
        };

        for (c, u) in master_deck {
            let card = if *u {
                g.new_card_upgraded(*c)
            } else {
                g.new_card(*c)
            };
            g.master_deck.push(card);
        }
        g.player.cur_hp = (g.player.cur_hp as f32 * 0.9) as i32;

        g
    }

    #[allow(dead_code)]
    pub fn set_debug(&mut self) {
        self.action_queue.set_debug();
    }

    fn new_card_id(&mut self, c: CardClass) -> u32 {
        if matches!(c, CardClass::RitualDagger | CardClass::Rampage) {
            let ret = self.next_id;
            self.next_id += 1;
            ret
        } else {
            0
        }
    }

    pub fn new_card(&mut self, class: CardClass) -> CardRef {
        let id = self.new_card_id(class);
        Rc::new(RefCell::new(Card {
            class,
            upgrade_count: 0,
            cost: class.base_cost(),
            exhaust: class.base_exhausts(),
            base_increase: 0,
            id,
        }))
    }

    pub fn new_card_upgraded(&mut self, class: CardClass) -> CardRef {
        let c = self.new_card(class);
        c.borrow_mut().upgrade();
        c
    }

    pub fn clone_card_ref_same_id(&self, c: &CardRef) -> CardRef {
        Rc::new(RefCell::new(c.borrow().clone()))
    }

    pub fn clone_card_ref_new_id(&mut self, c: &CardRef) -> CardRef {
        let mut c = c.borrow().clone();
        c.id = self.new_card_id(c.class);
        Rc::new(RefCell::new(c))
    }

    pub fn clone_card_new_id(&mut self, c: &Card) -> CardRef {
        let mut c = c.clone();
        c.id = self.new_card_id(c.class);
        Rc::new(RefCell::new(c))
    }

    pub fn get_creature(&self, r: CreatureRef) -> &Creature {
        match r.0 {
            0 => &self.player,
            r => &self.monsters[r - 1].creature,
        }
    }

    pub fn get_creature_mut(&mut self, r: CreatureRef) -> &mut Creature {
        match r.0 {
            0 => &mut self.player,
            r => &mut self.monsters[r - 1].creature,
        }
    }

    pub fn get_alive_monsters(&self) -> Vec<CreatureRef> {
        let mut alive = vec![];
        for (i, m) in self.monsters.iter().enumerate() {
            if !m.creature.is_alive() {
                continue;
            }
            alive.push(CreatureRef::monster(i));
        }
        alive
    }

    pub fn get_random_alive_monster(&mut self) -> CreatureRef {
        let alive = self.get_alive_monsters();
        rand_slice(&mut self.rng, &alive)
    }

    fn calculate_damage(
        &self,
        amount: i32,
        source_ref: CreatureRef,
        target_ref: CreatureRef,
    ) -> i32 {
        let mut amount_f = amount as f32;
        let source = self.get_creature(source_ref);
        let target = self.get_creature(target_ref);
        if let Some(s) = source.get_status(Status::Strength) {
            amount_f += s as f32;
        }
        if let Some(s) = source.get_status(Status::Vigor) {
            amount_f += s as f32;
        }
        if source.has_status(Status::Weak) {
            amount_f *= 0.75;
        }
        if source.has_status(Status::PenNib) {
            amount_f *= 2.0;
        }
        if target.has_status(Status::Vulnerable) {
            if target_ref.is_player() && self.has_relic(RelicClass::OddMushroom) {
                amount_f *= 1.25;
            } else if !target_ref.is_player() && self.has_relic(RelicClass::PaperPhrog) {
                amount_f *= 1.75;
            } else {
                amount_f *= 1.5;
            }
        }
        if target.has_status(Status::Intangible) {
            amount_f = amount_f.min(1.0);
        }
        0.max(amount_f as i32)
    }

    pub fn damage(&mut self, target: CreatureRef, mut amount: i32, ty: DamageType) {
        assert!(self.get_creature(target).is_alive());
        assert!(amount >= 0);
        if let DamageType::Attack {
            source,
            on_fatal: _,
        } = ty
        {
            if !self.get_creature(source).is_alive() {
                return;
            }
            amount = self.calculate_damage(amount, source, target);
            if let Some(a) = self
                .get_creature(target)
                .get_status(Status::Thorns)
                .map(|v| DamageAction::thorns_no_rupture(v, source))
            {
                self.action_queue.push_top(a);
            }
            if let Some(a) = self
                .get_creature(target)
                .get_status(Status::FlameBarrier)
                .map(|v| DamageAction::thorns_no_rupture(v, source))
            {
                self.action_queue.push_top(a);
            }
        }
        let c = self.get_creature_mut(target);
        if !c.is_alive() {
            return;
        }
        if ty != DamageType::HPLoss {
            if c.block >= amount {
                c.block -= amount;
                amount = 0;
            } else {
                amount -= c.block;
                c.block = 0;
            }
        }
        amount = amount.min(c.cur_hp);
        c.last_damage_taken = amount;
        if amount != 0 {
            c.cur_hp -= amount;
            // attack damage never procs rupture
            // hp loss always procs rupture
            // thorns proc rupture if source is player
            if matches!(
                ty,
                DamageType::HPLoss
                    | DamageType::Thorns {
                        procs_rupture: true
                    }
            ) && let Some(v) = self.get_creature(target).get_status(Status::Rupture)
            {
                self.action_queue.push_bot(GainStatusAction {
                    status: Status::Strength,
                    amount: v,
                    target,
                });
            }
            if matches!(ty, DamageType::Attack { .. })
                && self.get_creature(target).has_status(Status::PlatedArmor)
            {
                self.action_queue.push_bot(ReduceStatusAction {
                    status: Status::PlatedArmor,
                    amount: 1,
                    target,
                });
            }

            let update_blood_for_blood_cost = |cards: &[CardRef]| {
                for c in cards {
                    let mut c = c.borrow_mut();
                    if c.class == CardClass::BloodForBlood {
                        if let CardCost::Cost { base_cost, .. } = c.cost {
                            c.update_cost(0.max(base_cost - 1));
                        } else {
                            panic!();
                        }
                    }
                }
            };
            update_blood_for_blood_cost(&self.hand);
            update_blood_for_blood_cost(&self.discard_pile);
            update_blood_for_blood_cost(&self.draw_pile);
        }

        if !self.get_creature(target).is_alive()
            && target.is_player()
            && let Some(i) = self.potions.iter().position(|p| *p == Some(Potion::Fairy))
        {
            self.take_potion(i);
            let percent = if self.has_relic(RelicClass::SacredBark) {
                0.6
            } else {
                0.3
            };
            let amount = ((self.player.max_hp as f32 * percent) as i32).max(1);
            self.player.heal(amount);
        }
    }

    fn setup_combat_draw_pile(&mut self) {
        self.draw_pile = self
            .master_deck
            .iter()
            .map(|c| self.clone_card_ref_same_id(c))
            .collect();
        self.draw_pile.shuffle(&mut self.rng);
        self.draw_pile.sort_by_key(|c| c.borrow().is_innate());
        let num_innate = self
            .master_deck
            .iter()
            .filter(|c| c.borrow().is_innate())
            .count() as i32;
        let extra_draw = num_innate - self.draw_per_turn;
        if extra_draw > 0 {
            self.action_queue.push_bot(DrawAction(extra_draw));
        }
    }

    fn run_once(&mut self) {
        match self.state.cur_state() {
            GameState::RollCombat => {
                if let Some(m) = self.combat_monsters_queue.pop() {
                    self.monsters = m;
                    self.state.set_state(GameState::CombatBegin);
                } else {
                    self.state.set_state(GameState::Victory);
                }
            }
            GameState::CombatBegin => {
                self.turn = 0;

                self.setup_combat_draw_pile();

                // player pre-combat relic setup
                self.trigger_relics_at_pre_combat();
                self.run_actions_until_empty();

                // monster pre-combat setup
                for i in 0..self.monsters.len() {
                    if !self.monsters[i].creature.is_alive() {
                        continue;
                    }
                    self.monsters[i]
                        .behavior
                        .pre_combat(&mut self.action_queue, CreatureRef::monster(i));
                }
                self.run_actions_until_empty();

                self.state.set_state(GameState::PlayerTurnBegin);
            }
            GameState::PlayerTurnBegin => {
                self.monsters_roll_move();

                self.num_cards_played_this_turn = 0;

                self.player
                    .start_of_turn_lose_block(self.has_relic(RelicClass::Calipers));

                if self.turn == 0 {
                    self.trigger_relics_at_combat_start_pre_draw();
                }
                self.player
                    .trigger_statuses_turn_begin(CreatureRef::player(), &mut self.action_queue);

                self.action_queue.push_bot(DrawAction(self.draw_per_turn));

                if self.turn == 0 {
                    self.trigger_relics_at_combat_start_post_draw();
                }

                self.trigger_relics_at_turn_start();
                self.player.trigger_statuses_turn_begin_post_draw(
                    CreatureRef::player(),
                    &mut self.action_queue,
                );

                self.action_queue.push_top(StartOfTurnEnergyAction());

                self.run_actions_until_empty();

                self.state.set_state(GameState::PlayerTurn);
            }
            GameState::PlayerTurnEnd => {
                self.player_end_of_turn();
                if self.finished() {
                    return;
                }
                self.state.set_state(GameState::MonsterTurn);
            }
            GameState::MonsterTurn => {
                self.monsters_pre_turn();
                self.run_actions_until_empty();
                if self.finished() {
                    return;
                }

                self.monsters_take_turn();
                if self.finished() {
                    return;
                }

                self.monsters_end_turn();
                if self.finished() {
                    return;
                }
                self.state.set_state(GameState::EndOfRound);
            }
            GameState::EndOfRound => {
                self.player
                    .trigger_statuses_round_end(CreatureRef::player(), &mut self.action_queue);
                for (i, m) in self.monsters.iter_mut().enumerate() {
                    m.creature.trigger_statuses_round_end(
                        CreatureRef::monster(i),
                        &mut self.action_queue,
                    );
                }
                self.run_actions_until_empty();
                self.state.set_state(GameState::PlayerTurnBegin);
                self.turn += 1;
            }
            GameState::PlayerTurn => {
                self.run_actions_until_empty();
            }
            GameState::Victory
            | GameState::Defeat
            | GameState::Blessing
            | GameState::Transform
            | GameState::Armaments
            | GameState::PlaceCardInHandOnTopOfDraw
            | GameState::PlaceCardInDiscardOnTopOfDraw
            | GameState::ExhaustOneCardInHand
            | GameState::ExhaustCardsInHand { .. }
            | GameState::Memories { .. }
            | GameState::Gamble { .. }
            | GameState::Exhume
            | GameState::DualWield(_)
            | GameState::FetchCardFromDraw(_)
            | GameState::ForethoughtAny { .. }
            | GameState::ForethoughtOne
            | GameState::Discovery { .. } => {
                unreachable!()
            }
        }
    }

    fn run(&mut self) {
        self.run_actions_until_empty();
        if self.finished() {
            return;
        }
        while !matches!(
            self.state.cur_state(),
            GameState::PlayerTurn | GameState::Victory | GameState::Defeat
        ) && !self.in_pause_state()
        {
            self.run_once();
        }
    }

    fn in_pause_state(&self) -> bool {
        matches!(
            self.state.cur_state(),
            GameState::Armaments
                | GameState::ExhaustCardsInHand { .. }
                | GameState::Memories { .. }
                | GameState::PlaceCardInHandOnTopOfDraw
                | GameState::PlaceCardInDiscardOnTopOfDraw
                | GameState::ExhaustOneCardInHand
                | GameState::ForethoughtAny { .. }
                | GameState::Gamble { .. }
                | GameState::Exhume
                | GameState::DualWield(_)
                | GameState::Discovery { .. }
        )
    }

    fn run_actions_until_empty(&mut self) {
        loop {
            if let Some(a) = self.action_queue.pop() {
                a.run(self);
                if self.in_pause_state() {
                    break;
                }
            } else if !self.card_queue.is_empty() {
                let play = self.card_queue.remove(0);
                if self.can_play_card(&play) {
                    self.action_queue.push_bot(play);
                } else {
                    self.action_queue.push_bot(DiscardCardAction(play.card));
                }
            } else {
                break;
            }
        }
    }

    fn trigger_end_of_turn_cards_in_hand(&mut self) {
        let mut indexes_to_discard = Vec::new();
        let mut actions = vec![];
        for (i, c) in self.hand.iter().enumerate() {
            if let Some(a) = c.borrow().class.end_of_turn_in_hand_behavior() {
                indexes_to_discard.push(i);
                actions.push(a);
            }
        }
        for a in actions {
            a(self);
        }
        for i in indexes_to_discard.into_iter().rev() {
            self.action_queue
                .push_top(DiscardCardAction(self.hand.remove(i)));
        }
    }

    fn player_end_of_turn(&mut self) {
        self.trigger_relics_at_turn_end();
        self.player
            .trigger_statuses_turn_end(CreatureRef::player(), &mut self.action_queue);
        self.run_actions_until_empty();

        self.trigger_end_of_turn_cards_in_hand();
        self.run_actions_until_empty();

        self.action_queue.push_bot(EndOfTurnDiscardAction());
    }

    fn monsters_roll_move(&mut self) {
        let info = self.calculate_monster_info();
        for i in 0..self.monsters.len() {
            if !self.monsters[i].creature.is_alive() {
                continue;
            }
            self.monsters[i]
                .behavior
                .roll_next_action(&mut self.rng, &info);
        }
    }

    fn monsters_pre_turn(&mut self) {
        for i in 0..self.monsters.len() {
            if !self.monsters[i].creature.is_alive() {
                continue;
            }
            self.monsters[i].creature.start_of_turn_lose_block(false);
            self.monsters[i]
                .creature
                .trigger_statuses_turn_begin(CreatureRef::monster(i), &mut self.action_queue);
        }
    }

    fn monsters_take_turn(&mut self) {
        for i in 0..self.monsters.len() {
            let m = &mut self.monsters[i];
            if !m.creature.is_alive() {
                continue;
            }
            m.behavior
                .take_turn(CreatureRef::monster(i), &mut self.action_queue);
            self.run_actions_until_empty();
        }
    }

    fn monsters_end_turn(&mut self) {
        for i in 0..self.monsters.len() {
            if !self.monsters[i].creature.is_alive() {
                continue;
            }
            self.monsters[i]
                .creature
                .trigger_statuses_turn_end(CreatureRef::monster(i), &mut self.action_queue);
        }
        self.run_actions_until_empty();
    }

    pub fn remove_card_from_master_deck(&mut self, master_deck_index: usize) -> CardClass {
        let c = self.master_deck.remove(master_deck_index);
        let class = c.borrow().class;
        assert!(class.can_remove_from_master_deck());
        if class == CardClass::Parasite {
            self.action_queue.push_bot(DecreaseMaxHPAction(3));
            self.run_actions_until_empty();
        }
        class
    }

    fn transform_card_in_master_deck(&mut self, master_deck_index: usize) {
        let class = self.remove_card_from_master_deck(master_deck_index);
        let transformed = transformed(class, &mut self.rng);
        self.add_card_to_master_deck(transformed);
    }

    pub fn add_card_to_master_deck(&mut self, class: CardClass) {
        if class.ty() == CardType::Curse
            && let Some(v) = self.get_relic_value(RelicClass::Omamori)
            && v > 0
        {
            self.set_relic_value(RelicClass::Omamori, v - 1);
            return;
        }
        let c = self.new_card(class);
        self.master_deck.push(c);
    }

    pub fn result(&self) -> GameStatus {
        match self.state.cur_state() {
            GameState::Victory => GameStatus::Victory,
            GameState::Defeat => GameStatus::Defeat,
            &GameState::Memories {
                num_cards_remaining,
                ..
            } => GameStatus::Memories {
                num_cards_remaining,
            },
            &GameState::ExhaustCardsInHand {
                num_cards_remaining,
                ..
            } => GameStatus::ExhaustCardsInHand {
                num_cards_remaining,
            },
            _ => GameStatus::Combat,
        }
    }

    fn memories_cards(&mut self) {
        match self.state.cur_state_mut() {
            GameState::Memories {
                cards_to_memories, ..
            } => {
                while let Some(c) = cards_to_memories.pop() {
                    self.action_queue.push_top(MemoriesAction(c));
                }
            }
            _ => unreachable!(),
        }
        self.state.pop_state();
    }

    fn exhaust_cards(&mut self) {
        match &mut self.state.cur_state_mut() {
            GameState::ExhaustCardsInHand {
                cards_to_exhaust, ..
            } => {
                while let Some(c) = cards_to_exhaust.pop() {
                    self.action_queue.push_top(ExhaustCardAction(c));
                }
            }
            _ => unreachable!(),
        }
        self.state.pop_state();
    }

    fn gamble_cards(&mut self) {
        match self.state.cur_state_mut() {
            GameState::Gamble { cards_to_gamble } => {
                let count = cards_to_gamble.len() as i32;
                self.action_queue.push_top(DrawAction(count));
                while let Some(c) = cards_to_gamble.pop() {
                    self.action_queue.push_top(DiscardCardAction(c));
                }
            }
            _ => unreachable!(),
        }
        self.state.pop_state();
    }

    fn forethought_cards(&mut self) {
        match self.state.cur_state_mut() {
            GameState::ForethoughtAny {
                cards_to_forethought,
            } => {
                while !cards_to_forethought.is_empty() {
                    self.action_queue
                        .push_top(ForethoughtAction(cards_to_forethought.remove(0)));
                }
            }
            _ => unreachable!(),
        }
        self.state.pop_state();
    }

    pub fn make_move(&mut self, m: Move) {
        assert!(!self.finished());
        match m {
            Move::ChooseBlessing(b) => {
                assert_matches!(self.state.cur_state(), GameState::Blessing);
                self.state.set_state(GameState::RollCombat);
                b.run(self);
            }
            Move::Transform { card_index } => {
                assert_matches!(self.state.cur_state(), GameState::Transform);
                self.transform_card_in_master_deck(card_index);
                self.state.pop_state();
            }
            Move::EndTurn => {
                assert_matches!(self.state.cur_state(), GameState::PlayerTurn);
                self.state.set_state(GameState::PlayerTurnEnd);
            }
            Move::PlayCard { card_index, target } => {
                assert_matches!(self.state.cur_state(), GameState::PlayerTurn);
                let action = PlayCardAction {
                    card: self.hand.remove(card_index),
                    target: target.map(CreatureRef::monster),
                    is_duplicated: false,
                    energy: self.energy,
                    force_exhaust: false,
                    free: false,
                };
                assert!(self.can_play_card(&action));
                self.card_queue.push(action);
            }
            Move::Armaments { card_index } => {
                assert_matches!(self.state.cur_state(), GameState::Armaments);
                self.action_queue
                    .push_top(UpgradeAction(self.hand[card_index].clone()));
                self.state.pop_state();
            }
            Move::PlaceCardInHandOnTopOfDraw { card_index } => {
                assert_matches!(
                    self.state.cur_state(),
                    GameState::PlaceCardInHandOnTopOfDraw
                );
                self.action_queue
                    .push_top(PlaceCardOnTopOfDrawAction(self.hand.remove(card_index)));
                self.state.pop_state();
            }
            Move::PlaceCardInDiscardOnTopOfDraw { card_index } => {
                assert_matches!(
                    self.state.cur_state(),
                    GameState::PlaceCardInDiscardOnTopOfDraw
                );
                self.action_queue.push_top(PlaceCardOnTopOfDrawAction(
                    self.discard_pile.remove(card_index),
                ));
                self.state.pop_state();
            }
            Move::ExhaustOneCardInHand { card_index } => {
                assert_matches!(self.state.cur_state(), GameState::ExhaustOneCardInHand);
                self.action_queue
                    .push_top(ExhaustCardAction(self.hand.remove(card_index)));
                self.state.pop_state();
            }
            Move::DualWield { card_index } => {
                let amount = match self.state.cur_state() {
                    GameState::DualWield(amount) => *amount,
                    _ => panic!(),
                };
                self.action_queue.push_top(DualWieldAction {
                    card: self.hand.remove(card_index),
                    amount,
                    destroy_original: true,
                });
                self.state.pop_state();
            }
            Move::Exhume { card_index } => {
                assert_matches!(self.state.cur_state(), GameState::Exhume);
                self.action_queue
                    .push_top(PlaceCardInHandAction(self.exhaust_pile.remove(card_index)));
                self.state.pop_state();
            }
            Move::FetchCardFromDraw { card_index } => {
                assert!(matches!(
                    self.state.cur_state(),
                    GameState::FetchCardFromDraw(_)
                ));
                self.action_queue
                    .push_top(PlaceCardInHandAction(self.draw_pile.remove(card_index)));
                self.state.pop_state();
            }
            Move::Memories { card_index } => match self.state.cur_state_mut() {
                GameState::Memories {
                    num_cards_remaining,
                    cards_to_memories,
                } => {
                    *num_cards_remaining -= 1;
                    cards_to_memories.push(self.discard_pile.remove(card_index));
                    if *num_cards_remaining == 0 {
                        self.memories_cards();
                    }
                }
                _ => unreachable!(),
            },
            Move::ExhaustCardsInHand { card_index } => match self.state.cur_state_mut() {
                GameState::ExhaustCardsInHand {
                    num_cards_remaining,
                    cards_to_exhaust,
                } => {
                    *num_cards_remaining -= 1;
                    cards_to_exhaust.push(self.hand.remove(card_index));
                    if *num_cards_remaining == 0 || self.hand.is_empty() {
                        self.exhaust_cards();
                    }
                }
                _ => unreachable!(),
            },
            Move::ExhaustCardsInHandEnd => self.exhaust_cards(),
            Move::Gamble { card_index } => match self.state.cur_state_mut() {
                GameState::Gamble { cards_to_gamble } => {
                    cards_to_gamble.push(self.hand.remove(card_index));
                    if self.hand.is_empty() {
                        self.gamble_cards();
                    }
                }
                _ => unreachable!(),
            },
            Move::GambleEnd => self.gamble_cards(),
            Move::ForethoughtOne { card_index } => {
                assert_matches!(self.state.cur_state(), GameState::ForethoughtOne);
                self.action_queue
                    .push_top(ForethoughtAction(self.hand.remove(card_index)));
                self.state.pop_state();
            }
            Move::ForethoughtAny { card_index } => match self.state.cur_state_mut() {
                GameState::ForethoughtAny {
                    cards_to_forethought,
                } => {
                    cards_to_forethought.push(self.hand.remove(card_index));
                    if self.hand.is_empty() {
                        self.forethought_cards();
                    }
                }
                _ => unreachable!(),
            },
            Move::ForethoughtAnyEnd => self.forethought_cards(),
            Move::Discovery { card_class } => match self.state.cur_state() {
                &GameState::Discovery { amount, .. } => {
                    self.action_queue.push_top(DiscoveryAction {
                        class: card_class,
                        amount,
                    });
                    self.state.pop_state();
                }
                _ => unreachable!(),
            },
            Move::UsePotion {
                potion_index,
                target,
            } => {
                assert_matches!(self.state.cur_state(), GameState::PlayerTurn);
                let p = self.take_potion(potion_index);
                self.throw_potion(p, target.map(CreatureRef::monster));
            }
            Move::DiscardPotion { potion_index } => {
                assert_matches!(self.state.cur_state(), GameState::PlayerTurn);
                self.take_potion(potion_index);
            }
        }
        self.run();
    }

    pub fn can_play_card(&self, play: &PlayCardAction) -> bool {
        let c = play.card.borrow();
        let can_play_ty = match c.class.ty() {
            CardType::Attack => !self.player.has_status(Status::Entangled),
            CardType::Skill | CardType::Power => true,
            CardType::Status => {
                c.class == CardClass::Slimed || self.has_relic(RelicClass::MedicalKit)
            }
            CardType::Curse => self.has_relic(RelicClass::BlueCandle),
        };
        if !can_play_ty {
            return false;
        }
        let can_play_class = match c.class {
            CardClass::Clash => self
                .hand
                .iter()
                .all(|c| c.borrow().class.ty() == CardType::Attack),
            CardClass::SecretTechnique => self
                .draw_pile
                .iter()
                .any(|c| c.borrow().class.ty() == CardType::Skill),
            CardClass::SecretWeapon => self
                .draw_pile
                .iter()
                .any(|c| c.borrow().class.ty() == CardType::Attack),
            _ => true,
        };
        if !can_play_class {
            return false;
        }
        if self.num_cards_played_this_turn >= 3
            && self
                .hand
                .iter()
                .any(|c| c.borrow().class == CardClass::Normality)
        {
            return false;
        }
        self.energy >= play.cost(self)
    }

    pub fn valid_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        match self.state.cur_state() {
            GameState::Blessing => {
                moves.push(Move::ChooseBlessing(Blessing::GainMaxHPSmall));
                moves.push(Move::ChooseBlessing(Blessing::CommonRelic));
                moves.push(Move::ChooseBlessing(Blessing::RemoveRelic));
                moves.push(Move::ChooseBlessing(Blessing::TransformOne));
                moves.push(Move::ChooseBlessing(Blessing::RandomUncommonColorless));
                moves.push(Move::ChooseBlessing(Blessing::RandomPotion));
            }
            GameState::Transform => {
                for (i, c) in self.master_deck.iter().enumerate() {
                    if c.borrow().class.can_remove_from_master_deck() {
                        moves.push(Move::Transform { card_index: i });
                    }
                }
            }
            GameState::PlayerTurn => {
                moves.push(Move::EndTurn);
                for (ci, c) in self.hand.iter().enumerate() {
                    if !self.can_play_card(&PlayCardAction {
                        card: c.clone(),
                        target: None,
                        is_duplicated: false,
                        energy: self.energy,
                        free: false,
                        force_exhaust: false,
                    }) {
                        continue;
                    }
                    let c = c.borrow();
                    if c.has_target() {
                        for (mi, m) in self.monsters.iter().enumerate() {
                            if !m.creature.is_alive() {
                                continue;
                            }
                            moves.push(Move::PlayCard {
                                card_index: ci,
                                target: Some(mi),
                            });
                        }
                    } else {
                        moves.push(Move::PlayCard {
                            card_index: ci,
                            target: None,
                        });
                    }
                }
                for (pi, p) in self.potions.iter().enumerate() {
                    if let Some(p) = p {
                        moves.push(Move::DiscardPotion { potion_index: pi });
                        if !p.can_use() {
                            continue;
                        }
                        if p.has_target() {
                            for (mi, m) in self.monsters.iter().enumerate() {
                                if !m.creature.is_alive() {
                                    continue;
                                }
                                moves.push(Move::UsePotion {
                                    potion_index: pi,
                                    target: Some(mi),
                                });
                            }
                        } else {
                            moves.push(Move::UsePotion {
                                potion_index: pi,
                                target: None,
                            });
                        }
                    }
                }
            }
            GameState::Armaments => {
                for (i, c) in self.hand.iter().enumerate() {
                    if c.borrow().can_upgrade() {
                        moves.push(Move::Armaments { card_index: i });
                    }
                }
            }
            GameState::ForethoughtOne => {
                for i in 0..self.hand.len() {
                    moves.push(Move::ForethoughtOne { card_index: i });
                }
            }
            GameState::ForethoughtAny { .. } => {
                moves.push(Move::ForethoughtAnyEnd);
                for c in 0..self.hand.len() {
                    moves.push(Move::ForethoughtAny { card_index: c });
                }
            }
            GameState::PlaceCardInHandOnTopOfDraw => {
                for i in 0..self.hand.len() {
                    moves.push(Move::PlaceCardInHandOnTopOfDraw { card_index: i });
                }
            }
            GameState::PlaceCardInDiscardOnTopOfDraw => {
                for i in 0..self.discard_pile.len() {
                    moves.push(Move::PlaceCardInDiscardOnTopOfDraw { card_index: i });
                }
            }
            GameState::ExhaustOneCardInHand => {
                for i in 0..self.hand.len() {
                    moves.push(Move::ExhaustOneCardInHand { card_index: i });
                }
            }
            GameState::Exhume => {
                for (i, c) in self.exhaust_pile.iter().enumerate() {
                    if c.borrow().class != CardClass::Exhume {
                        moves.push(Move::Exhume { card_index: i });
                    }
                }
            }
            GameState::DualWield(_) => {
                for (i, c) in self.hand.iter().enumerate() {
                    if can_dual_wield(c) {
                        moves.push(Move::DualWield { card_index: i });
                    }
                }
            }
            GameState::FetchCardFromDraw(ty) => {
                for (i, c) in self.draw_pile.iter().enumerate() {
                    if c.borrow().class.ty() == *ty {
                        moves.push(Move::FetchCardFromDraw { card_index: i });
                    }
                }
            }
            GameState::ExhaustCardsInHand { .. } => {
                moves.push(Move::ExhaustCardsInHandEnd);
                for c in 0..self.hand.len() {
                    moves.push(Move::ExhaustCardsInHand { card_index: c });
                }
            }
            GameState::Memories { .. } => {
                for c in 0..self.discard_pile.len() {
                    moves.push(Move::Memories { card_index: c });
                }
            }
            GameState::Gamble { .. } => {
                moves.push(Move::GambleEnd);
                for c in 0..self.hand.len() {
                    moves.push(Move::Gamble { card_index: c });
                }
            }
            GameState::Discovery { classes, .. } => {
                for &card_class in classes {
                    moves.push(Move::Discovery { card_class })
                }
            }
            _ => {
                unreachable!();
            }
        }
        assert!(!moves.is_empty());
        moves
    }

    pub fn should_add_extra_decay_status(&self) -> bool {
        matches!(
            self.state.cur_state(),
            GameState::MonsterTurn | GameState::PlayerTurnEnd
        )
    }

    #[cfg(test)]
    pub fn run_action<A: Action + 'static>(&mut self, a: A) {
        self.action_queue.push_bot(a);
        self.run_actions_until_empty();
    }

    pub fn throw_potion(&mut self, p: Potion, target: Option<CreatureRef>) {
        let is_sacred = self.has_relic(RelicClass::SacredBark);
        p.behavior()(is_sacred, target, self);
        self.run();
    }

    #[cfg(test)]
    fn play_card_impl(&mut self, card: CardRef, target: Option<CreatureRef>) {
        let action = PlayCardAction {
            card,
            target,
            is_duplicated: false,
            energy: self.energy,
            force_exhaust: false,
            free: false,
        };
        assert!(self.can_play_card(&action));
        self.card_queue.push(action);
        self.run_actions_until_empty();
    }

    #[cfg(test)]
    pub fn play_card(&mut self, class: CardClass, target: Option<CreatureRef>) {
        let card = self.new_card(class);
        self.play_card_impl(card, target);
    }

    #[cfg(test)]
    pub fn play_card_upgraded(&mut self, class: CardClass, target: Option<CreatureRef>) {
        let card = self.new_card_upgraded(class);
        self.play_card_impl(card, target);
    }

    #[cfg(test)]
    pub fn add_card_to_hand(&mut self, class: CardClass) {
        let card = self.new_card(class);
        self.hand.push(card);
    }

    #[cfg(test)]
    pub fn add_card_to_hand_upgraded(&mut self, class: CardClass) {
        let card = self.new_card_upgraded(class);
        self.hand.push(card);
    }

    #[cfg(test)]
    pub fn add_card_to_draw_pile(&mut self, class: CardClass) {
        let card = self.new_card(class);
        self.draw_pile.push(card);
    }

    #[cfg(test)]
    pub fn add_card_to_discard_pile(&mut self, class: CardClass) {
        let card = self.new_card(class);
        self.discard_pile.push(card);
    }

    #[cfg(test)]
    pub fn add_cards_to_discard_pile(&mut self, class: CardClass, amount: i32) {
        for _ in 0..amount {
            self.add_card_to_discard_pile(class);
        }
    }

    #[cfg(test)]
    pub fn add_card_to_exhaust_pile(&mut self, class: CardClass) {
        let card = self.new_card(class);
        self.exhaust_pile.push(card);
    }

    #[cfg(test)]
    pub fn clear_all_piles(&mut self) {
        self.hand.clear();
        self.discard_pile.clear();
        self.draw_pile.clear();
        self.exhaust_pile.clear();
    }

    pub fn hand_is_full(&self) -> bool {
        self.hand.len() as i32 == Game::MAX_HAND_SIZE
    }

    fn finished(&mut self) -> bool {
        assert_not_matches!(
            self.state.cur_state(),
            GameState::Defeat | GameState::Victory
        );
        if !self.player.is_alive() {
            self.state.set_state(GameState::Defeat);
            return true;
        }
        if self.combat_monsters_queue.is_empty()
            && self.monsters.iter().all(|m| !m.creature.is_alive())
        {
            self.state.set_state(GameState::Victory);
            self.trigger_relics_at_combat_finish();
            self.run_actions_until_empty();
            return true;
        }
        false
    }

    fn calculate_monster_info(&self) -> MonsterInfo {
        MonsterInfo {
            num_monsters: self.monsters.len(),
        }
    }

    pub fn heal(&mut self, c: CreatureRef, amount: i32) {
        if amount == 0 {
            return;
        }
        // check player healing relics
        self.get_creature_mut(c).heal(amount);
        // trigger player on heal relics
    }

    pub fn increase_max_hp(&mut self, amount: i32) {
        self.player.increase_max_hp(amount);
        self.heal(CreatureRef::player(), amount);
    }

    pub fn add_potion(&mut self, potion: Potion) {
        let mut added = false;
        for p in &mut self.potions {
            if p.is_none() {
                *p = Some(potion);
                added = true;
                break;
            }
        }
        assert!(added);
    }
    pub fn take_potion(&mut self, i: usize) -> Potion {
        let p = self.potions[i].unwrap();
        self.potions[i] = None;
        p
    }
    pub fn add_relic(&mut self, class: RelicClass) {
        let mut r = new_relic(class);
        r.on_equip(&mut self.action_queue);
        self.relics.push(r);
        self.run_actions_until_empty();
    }
    pub fn remove_relic(&mut self, class: RelicClass) {
        let idx = self.relics.iter().position(|r| r.get_class() == class);
        let mut r = self.relics.remove(idx.unwrap());
        r.on_unequip(&mut self.action_queue);
        self.run_actions_until_empty();
    }
    pub fn has_relic(&self, class: RelicClass) -> bool {
        self.relics.iter().any(|r| r.get_class() == class)
    }
    pub fn get_relic_value(&self, class: RelicClass) -> Option<i32> {
        self.relics
            .iter()
            .find(|r| r.get_class() == class)
            .map(|r| r.get_value())
    }
    pub fn set_relic_value(&mut self, class: RelicClass, v: i32) {
        self.relics
            .iter_mut()
            .find(|r| r.get_class() == class)
            .unwrap()
            .set_value(v);
    }
    trigger!(trigger_relics_on_shuffle, on_shuffle);
    trigger!(trigger_relics_at_pre_combat, at_pre_combat);
    trigger!(
        trigger_relics_at_combat_start_pre_draw,
        at_combat_start_pre_draw
    );
    trigger!(
        trigger_relics_at_combat_start_post_draw,
        at_combat_start_post_draw
    );
    trigger!(trigger_relics_at_turn_start, at_turn_start);
    trigger!(trigger_relics_at_turn_end, at_turn_end);
    trigger!(trigger_relics_at_combat_finish, at_combat_finish);
    trigger_card!(trigger_relics_on_card_played, on_card_played);
}

#[cfg(test)]
mod tests {
    use crate::{actions::block::BlockAction, monsters::test::AttackMonster, potion::Potion};

    use super::*;

    #[test]
    fn test_moves() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .build_combat();
        g.add_card_to_hand(CardClass::DebugKill);
        g.add_card_to_hand(CardClass::Defend);
        g.add_potion(Potion::Fire);
        g.add_potion(Potion::Flex);
        assert_eq!(
            g.valid_moves()
                .iter()
                .filter(|m| matches!(m, Move::EndTurn))
                .count(),
            1
        );
        assert_eq!(
            g.valid_moves()
                .iter()
                .filter(|m| matches!(
                    m,
                    Move::PlayCard {
                        card_index: _,
                        target: _
                    }
                ))
                .count(),
            3
        );
        assert_eq!(
            g.valid_moves()
                .iter()
                .filter(|m| matches!(
                    m,
                    Move::UsePotion {
                        potion_index: _,
                        target: _
                    }
                ))
                .count(),
            3
        );

        g.run_action(DamageAction::thorns_rupture(9999, CreatureRef::monster(0)));
        assert_eq!(
            g.valid_moves()
                .iter()
                .filter(|m| matches!(m, Move::EndTurn))
                .count(),
            1
        );
        assert_eq!(
            g.valid_moves()
                .iter()
                .filter(|m| matches!(
                    m,
                    Move::PlayCard {
                        card_index: _,
                        target: _
                    }
                ))
                .count(),
            2
        );
        assert_eq!(
            g.valid_moves()
                .iter()
                .filter(|m| matches!(
                    m,
                    Move::UsePotion {
                        potion_index: _,
                        target: _
                    }
                ))
                .count(),
            2
        );
    }

    #[test]
    fn test_player_lose_block_start_of_turn() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(BlockAction::player_flat_amount(7));
        assert_eq!(g.player.block, 7);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 0);
    }

    #[test]
    fn test_monster_lose_block_start_of_turn() {
        let mut g = GameBuilder::default().build_combat();
        g.run_action(BlockAction::monster(CreatureRef::monster(0), 7));
        assert_eq!(g.monsters[0].creature.block, 7);
        g.make_move(Move::EndTurn);
        assert_eq!(g.monsters[0].creature.block, 0);
    }

    #[test]
    fn test_barricade() {
        let mut g = GameBuilder::default()
            .add_monster_status(Status::Barricade, 1)
            .add_player_status(Status::Barricade, 1)
            .build_combat();
        g.run_action(BlockAction::monster(CreatureRef::player(), 7));
        g.run_action(BlockAction::monster(CreatureRef::monster(0), 7));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 7);
        assert_eq!(g.monsters[0].creature.block, 7);
    }

    #[test]
    fn test_unplayable_card_in_card_queue() {
        let mut g = GameBuilder::default()
            .add_player_status(Status::Entangled, 1)
            .build_combat();
        let c = g.new_card(CardClass::Thunderclap);
        g.card_queue.push(PlayCardAction {
            card: c,
            target: None,
            is_duplicated: false,
            energy: g.energy,
            free: false,
            force_exhaust: false,
        });
        g.run_actions_until_empty();
        assert_eq!(g.discard_pile.len(), 1);
    }

    #[test]
    fn test_free_to_play() {
        let mut g = GameBuilder::default().build_combat();
        let c = g.new_card(CardClass::Defend);
        match &mut c.borrow_mut().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => *free_to_play_once = true,
            _ => panic!(),
        }
        g.hand.push(c);
        assert_eq!(g.energy, 3);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
        g.hand.push(g.discard_pile.pop().unwrap());
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 2);

        g.make_move(Move::EndTurn);
        assert_eq!(g.energy, 3);
        match &mut g.hand[0].borrow_mut().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => *free_to_play_once = true,
            _ => panic!(),
        }
        g.make_move(Move::EndTurn);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
    }

    #[test]
    fn test_has_relic() {
        use RelicClass::{BagOfPrep, BloodVial};
        let mut g = GameBuilder::default().build();

        assert!(!g.has_relic(BagOfPrep));
        assert!(!g.has_relic(BloodVial));

        g.add_relic(BagOfPrep);
        assert!(g.has_relic(BagOfPrep));
        assert!(!g.has_relic(BloodVial));

        g.remove_relic(BagOfPrep);
        assert!(!g.has_relic(BagOfPrep));
        assert!(!g.has_relic(BloodVial));
    }

    #[test]
    fn test_potions() {
        use Potion::{Attack, Skill};
        let mut g = GameBuilder::default().build();
        assert_eq!(g.potions, vec![None, None]);

        g.add_potion(Attack);
        assert_eq!(g.potions, vec![Some(Attack), None]);

        g.add_potion(Skill);
        assert_eq!(g.potions, vec![Some(Attack), Some(Skill)]);

        assert_eq!(g.take_potion(0), Attack);
        assert_eq!(g.potions, vec![None, Some(Skill)]);

        g.add_potion(Attack);
        assert_eq!(g.potions, vec![Some(Attack), Some(Skill)]);

        assert_eq!(g.take_potion(1), Skill);
        assert_eq!(g.potions, vec![Some(Attack), None]);
    }

    #[test]
    fn test_multi_attack_die_to_thorns() {
        let mut g = GameBuilder::default()
            .add_monster(AttackMonster::with_attack_count(10, 10))
            .add_player_status(Status::Thorns, 999)
            .build_combat();
        g.player.cur_hp = 50;
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.cur_hp, 40);
    }
}
