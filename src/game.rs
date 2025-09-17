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
use crate::monster::{Monster, MonsterBehavior, MonsterInfo};
use crate::monsters::test::NoopMonster;
use crate::player::Player;
use crate::potion::Potion;
use crate::queue::ActionQueue;
use crate::relic::RelicClass;
use crate::rng::rand_slice;
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

#[derive(Debug)]
pub enum GameState {
    Blessing,
    BlessingTransform,
    RollCombat,
    CombatBegin,
    PlayerTurnBegin,
    PlayerTurn,
    PlayerTurnEnd,
    MonsterTurn,
    EndOfRound,
    Armaments,
    Memories {
        num_cards_remaining: i32,
        cards_to_memories: Vec<CardRef>,
    },
    ExhaustOneCardInHand,
    ExhaustCardsInHand {
        num_cards_remaining: i32,
        cards_to_exhaust: Vec<CardRef>,
    },
    Gamble {
        cards_to_gamble: Vec<CardRef>,
    },
    PlaceCardInHandOnTopOfDraw,
    PlaceCardInDiscardOnTopOfDraw,
    Exhume,
    DualWield(i32),
    FetchCardFromDraw(CardType),
    ForethoughtAny {
        cards_to_forethought: Vec<CardRef>,
    },
    ForethoughtOne,
    Discovery {
        classes: Vec<CardClass>,
        amount: i32,
    },
    Defeat,
    Victory,
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
        let monster_statuses = self.monster_statuses.clone();
        let mut g = self.build();
        g.state = GameState::RollCombat;
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
            g.player.creature.set_status(k, v);
        }
        for r in self.relics {
            g.player.add_relic(r);
        }
        if let Some(hp) = self.player_hp {
            g.player.creature.cur_hp = hp;
        }
        g
    }
}

pub struct Game {
    pub player: Player,
    pub combat_monsters_queue: Vec<Vec<Monster>>,
    pub monsters: Vec<Monster>,
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
    pub rng: Rand,
    pub state: GameState,
    next_id: u32,
}

impl Game {
    pub const MAX_HAND_SIZE: i32 = 10;

    fn new(rng: Rand, master_deck: &[(CardClass, bool)], monsters: Vec<Monster>) -> Self {
        let mut g = Self {
            player: Player::new("Ironclad", 80),
            combat_monsters_queue: vec![monsters],
            monsters: Default::default(),
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
            rng,
            state: GameState::Blessing,
            next_id: 1,
        };

        for (c, u) in master_deck {
            let card = if *u {
                g.new_card_upgraded(*c)
            } else {
                g.new_card(*c)
            };
            g.player.master_deck.push(card);
        }
        g.player.creature.cur_hp = (g.player.creature.cur_hp as f32 * 0.9) as i32;

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
            0 => &self.player.creature,
            r => &self.monsters[r - 1].creature,
        }
    }

    pub fn get_creature_mut(&mut self, r: CreatureRef) -> &mut Creature {
        match r.0 {
            0 => &mut self.player.creature,
            r => &mut self.monsters[r - 1].creature,
        }
    }

    pub fn get_random_alive_monster(&mut self) -> CreatureRef {
        let mut alive = vec![];
        for (i, m) in self.monsters.iter().enumerate() {
            if !m.creature.is_alive() {
                continue;
            }
            alive.push(i);
        }
        CreatureRef::monster(rand_slice(&mut self.rng, &alive))
    }

    pub fn damage(&mut self, target: CreatureRef, mut amount: i32, ty: DamageType) {
        assert!(self.get_creature(target).is_alive());
        assert!(amount >= 0);
        if let DamageType::Attack {
            source,
            on_fatal: _,
        } = ty
        {
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
            && let Some(i) = self
                .player
                .potions
                .iter()
                .position(|p| *p == Some(Potion::Fairy))
        {
            self.player.take_potion(i);
            let percent = if self.player.has_relic(RelicClass::SacredBark) {
                0.6
            } else {
                0.3
            };
            let amount = ((self.player.creature.max_hp as f32 * percent) as i32).max(1);
            self.player.creature.heal(amount);
        }
    }

    fn setup_combat_draw_pile(&mut self) {
        self.draw_pile = self
            .player
            .master_deck
            .iter()
            .map(|c| self.clone_card_ref_same_id(c))
            .collect();
        self.draw_pile.shuffle(&mut self.rng);
        self.draw_pile.sort_by_key(|c| c.borrow().is_innate());
        let num_innate = self
            .player
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
        match self.state {
            GameState::RollCombat => {
                if let Some(m) = self.combat_monsters_queue.pop() {
                    self.monsters = m;
                    self.state = GameState::CombatBegin;
                } else {
                    self.state = GameState::Victory;
                }
            }
            GameState::CombatBegin => {
                self.turn = 0;

                self.setup_combat_draw_pile();

                // player pre-combat relic setup
                self.player
                    .trigger_relics_at_pre_combat(&mut self.action_queue);
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

                self.state = GameState::PlayerTurnBegin;
            }
            GameState::PlayerTurnBegin => {
                self.monsters_roll_move();

                self.num_cards_played_this_turn = 0;

                self.action_queue.push_top(StartOfTurnEnergyAction());
                self.player.creature.start_of_turn_lose_block();

                if self.turn == 0 {
                    self.player
                        .trigger_relics_at_combat_start_pre_draw(&mut self.action_queue);
                }
                self.player
                    .creature
                    .trigger_statuses_turn_begin(CreatureRef::player(), &mut self.action_queue);

                self.action_queue.push_bot(DrawAction(self.draw_per_turn));

                if self.turn == 0 {
                    self.player
                        .trigger_relics_at_combat_start_post_draw(&mut self.action_queue);
                }

                self.player
                    .trigger_relics_at_turn_start(&mut self.action_queue);
                self.player.creature.trigger_statuses_turn_begin_post_draw(
                    CreatureRef::player(),
                    &mut self.action_queue,
                );

                self.run_actions_until_empty();

                self.state = GameState::PlayerTurn;
            }
            GameState::PlayerTurnEnd => {
                self.player_end_of_turn();
                if self.finished() {
                    return;
                }
                self.state = GameState::MonsterTurn;
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
                self.state = GameState::EndOfRound;
            }
            GameState::EndOfRound => {
                self.player
                    .creature
                    .trigger_statuses_round_end(CreatureRef::player(), &mut self.action_queue);
                for (i, m) in self.monsters.iter_mut().enumerate() {
                    m.creature.trigger_statuses_round_end(
                        CreatureRef::monster(i),
                        &mut self.action_queue,
                    );
                }
                self.run_actions_until_empty();
                self.state = GameState::PlayerTurnBegin;
                self.turn += 1;
            }
            GameState::PlayerTurn => {
                self.run_actions_until_empty();
            }
            GameState::Victory
            | GameState::Defeat
            | GameState::Blessing
            | GameState::BlessingTransform
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
            self.state,
            GameState::PlayerTurn | GameState::Victory | GameState::Defeat
        ) && !self.in_pause_state()
        {
            self.run_once();
        }
    }

    fn in_pause_state(&self) -> bool {
        matches!(
            self.state,
            GameState::Armaments
                | GameState::ExhaustCardsInHand { .. }
                | GameState::Memories { .. }
                | GameState::PlaceCardInHandOnTopOfDraw
                | GameState::PlaceCardInDiscardOnTopOfDraw
                | GameState::ExhaustOneCardInHand
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
        self.player
            .trigger_relics_at_turn_end(&mut self.action_queue);
        self.player
            .creature
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
            self.monsters[i].creature.start_of_turn_lose_block();
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
            m.behavior.take_turn(
                &mut self.action_queue,
                &self.player,
                &m.creature,
                CreatureRef::monster(i),
            );
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
        let c = self.player.master_deck.remove(master_deck_index);
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
        let c = self.new_card(class);
        self.player.master_deck.push(c);
    }

    pub fn result(&self) -> GameStatus {
        match &self.state {
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
        match &mut self.state {
            GameState::Memories {
                cards_to_memories, ..
            } => {
                while let Some(c) = cards_to_memories.pop() {
                    self.action_queue.push_top(MemoriesAction(c));
                }
            }
            _ => unreachable!(),
        }
        self.state = GameState::PlayerTurn;
        self.run();
    }

    fn exhaust_cards(&mut self) {
        match &mut self.state {
            GameState::ExhaustCardsInHand {
                cards_to_exhaust, ..
            } => {
                while let Some(c) = cards_to_exhaust.pop() {
                    self.action_queue.push_top(ExhaustCardAction(c));
                }
            }
            _ => unreachable!(),
        }
        self.state = GameState::PlayerTurn;
        self.run();
    }

    fn gamble_cards(&mut self) {
        match &mut self.state {
            GameState::Gamble { cards_to_gamble } => {
                let count = cards_to_gamble.len() as i32;
                self.action_queue.push_top(DrawAction(count));
                while let Some(c) = cards_to_gamble.pop() {
                    self.action_queue.push_top(DiscardCardAction(c));
                }
            }
            _ => unreachable!(),
        }
        self.state = GameState::PlayerTurn;
        self.run();
    }

    fn forethought_cards(&mut self) {
        match &mut self.state {
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
        self.state = GameState::PlayerTurn;
        self.run();
    }

    pub fn make_move(&mut self, m: Move) {
        assert!(!self.finished());
        match m {
            Move::ChooseBlessing(b) => {
                assert_matches!(self.state, GameState::Blessing);
                b.run(self);
                if !matches!(self.state, GameState::BlessingTransform) {
                    self.state = GameState::RollCombat;
                    self.run();
                }
            }
            Move::Transform { card_index } => {
                assert_matches!(self.state, GameState::BlessingTransform);
                self.transform_card_in_master_deck(card_index);
                self.state = GameState::RollCombat;
                self.run();
            }
            Move::EndTurn => {
                assert_matches!(self.state, GameState::PlayerTurn);
                self.state = GameState::PlayerTurnEnd;
                self.run();
            }
            Move::PlayCard { card_index, target } => {
                assert_matches!(self.state, GameState::PlayerTurn);
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
                self.run();
            }
            Move::Armaments { card_index } => {
                assert_matches!(self.state, GameState::Armaments);
                self.action_queue
                    .push_top(UpgradeAction(self.hand[card_index].clone()));
                self.state = GameState::PlayerTurn;
                self.run();
            }
            Move::PlaceCardInHandOnTopOfDraw { card_index } => {
                assert_matches!(self.state, GameState::PlaceCardInHandOnTopOfDraw);
                self.action_queue
                    .push_top(PlaceCardOnTopOfDrawAction(self.hand.remove(card_index)));
                self.state = GameState::PlayerTurn;
                self.run();
            }
            Move::PlaceCardInDiscardOnTopOfDraw { card_index } => {
                assert_matches!(self.state, GameState::PlaceCardInDiscardOnTopOfDraw);
                self.action_queue.push_top(PlaceCardOnTopOfDrawAction(
                    self.discard_pile.remove(card_index),
                ));
                self.state = GameState::PlayerTurn;
                self.run();
            }
            Move::ExhaustOneCardInHand { card_index } => {
                assert_matches!(self.state, GameState::ExhaustOneCardInHand);
                self.action_queue
                    .push_top(ExhaustCardAction(self.hand.remove(card_index)));
                self.state = GameState::PlayerTurn;
                self.run();
            }
            Move::DualWield { card_index } => {
                let amount = match self.state {
                    GameState::DualWield(amount) => amount,
                    _ => panic!(),
                };
                self.action_queue.push_top(DualWieldAction {
                    card: self.hand.remove(card_index),
                    amount,
                    destroy_original: true,
                });
                self.state = GameState::PlayerTurn;
                self.run();
            }
            Move::Exhume { card_index } => {
                assert_matches!(self.state, GameState::Exhume);
                self.action_queue
                    .push_top(PlaceCardInHandAction(self.exhaust_pile.remove(card_index)));
                self.state = GameState::PlayerTurn;
                self.run();
            }
            Move::FetchCardFromDraw { card_index } => {
                assert!(matches!(self.state, GameState::FetchCardFromDraw(_)));
                self.action_queue
                    .push_top(PlaceCardInHandAction(self.draw_pile.remove(card_index)));
                self.state = GameState::PlayerTurn;
                self.run();
            }
            Move::Memories { card_index } => match &mut self.state {
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
            Move::ExhaustCardsInHand { card_index } => match &mut self.state {
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
            Move::Gamble { card_index } => match &mut self.state {
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
                assert_matches!(self.state, GameState::ForethoughtOne);
                self.action_queue
                    .push_top(ForethoughtAction(self.hand.remove(card_index)));
                self.state = GameState::PlayerTurn;
                self.run();
            }
            Move::ForethoughtAny { card_index } => match &mut self.state {
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
            Move::Discovery { card_class } => match self.state {
                GameState::Discovery { amount, .. } => {
                    self.action_queue.push_top(DiscoveryAction {
                        class: card_class,
                        amount,
                    });
                    self.state = GameState::PlayerTurn;
                    self.run();
                }
                _ => unreachable!(),
            },
            Move::UsePotion {
                potion_index,
                target,
            } => {
                assert_matches!(self.state, GameState::PlayerTurn);
                let p = self.player.take_potion(potion_index);
                self.throw_potion(p, target.map(CreatureRef::monster));
                self.run();
            }
            Move::DiscardPotion { potion_index } => {
                assert_matches!(self.state, GameState::PlayerTurn);
                self.player.take_potion(potion_index);
            }
        }
    }

    pub fn can_play_card(&self, play: &PlayCardAction) -> bool {
        let c = play.card.borrow();
        let can_play_ty = match c.class.ty() {
            CardType::Attack => !self.player.creature.has_status(Status::Entangled),
            CardType::Skill | CardType::Power => true,
            CardType::Status => {
                c.class == CardClass::Slimed || self.player.has_relic(RelicClass::MedicalKit)
            }
            CardType::Curse => self.player.has_relic(RelicClass::BlueCandle),
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
        match &self.state {
            GameState::Blessing => {
                moves.push(Move::ChooseBlessing(Blessing::GainMaxHPSmall));
                moves.push(Move::ChooseBlessing(Blessing::CommonRelic));
                moves.push(Move::ChooseBlessing(Blessing::TransformOne));
                moves.push(Move::ChooseBlessing(Blessing::RandomUncommonColorless));
                moves.push(Move::ChooseBlessing(Blessing::RandomPotion));
            }
            GameState::BlessingTransform => {
                for (i, c) in self.player.master_deck.iter().enumerate() {
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
                for (pi, p) in self.player.potions.iter().enumerate() {
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
            self.state,
            GameState::MonsterTurn | GameState::PlayerTurnEnd
        )
    }

    #[cfg(test)]
    pub fn run_action<A: Action + 'static>(&mut self, a: A) {
        self.action_queue.push_bot(a);
        self.run_actions_until_empty();
    }

    pub fn throw_potion(&mut self, p: Potion, target: Option<CreatureRef>) {
        let is_sacred = self.player.has_relic(RelicClass::SacredBark);
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
        assert_not_matches!(self.state, GameState::Defeat | GameState::Victory);
        if !self.player.creature.is_alive() {
            self.state = GameState::Defeat;
            return true;
        }
        if self.combat_monsters_queue.is_empty()
            && self.monsters.iter().all(|m| !m.creature.is_alive())
        {
            self.state = GameState::Victory;
            self.player
                .trigger_relics_at_combat_finish(&mut self.action_queue);
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
        self.player.creature.increase_max_hp(amount);
        self.heal(CreatureRef::player(), amount);
    }
}

#[cfg(test)]
mod tests {
    use crate::{actions::block::BlockAction, potion::Potion};

    use super::*;

    #[test]
    fn test_moves() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .build_combat();
        g.add_card_to_hand(CardClass::DebugKill);
        g.add_card_to_hand(CardClass::Defend);
        g.player.add_potion(Potion::Fire);
        g.player.add_potion(Potion::Flex);
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
        assert_eq!(g.player.creature.block, 7);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
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
        assert_eq!(g.player.creature.block, 7);
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
}
