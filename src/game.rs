use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]
use crate::action::Action;
use crate::actions::add_card_to_master_deck::AddCardToMasterDeckAction;
use crate::actions::choose_dual_wield::can_dual_wield;
use crate::actions::damage::{DamageAction, DamageType};
use crate::actions::discard_card::DiscardCardAction;
use crate::actions::discovery::DiscoveryAction;
use crate::actions::draw::DrawAction;
use crate::actions::dual_wield::DualWieldAction;
use crate::actions::end_of_turn_discard::EndOfTurnDiscardAction;
use crate::actions::exhaust_card::ExhaustCardAction;
use crate::actions::forethought::ForethoughtAction;
use crate::actions::gain_energy::GainEnergyAction;
use crate::actions::gain_status::GainStatusAction;
use crate::actions::memories::MemoriesAction;
use crate::actions::place_card_in_hand::PlaceCardInHandAction;
use crate::actions::place_card_on_top_of_draw::PlaceCardOnTopOfDrawAction;
use crate::actions::play_card::PlayCardAction;
use crate::actions::reduce_status::ReduceStatusAction;
use crate::actions::removed_card_from_master_deck::RemovedCardFromMasterDeckAction;
use crate::actions::start_of_turn_energy::StartOfTurnEnergyAction;
use crate::actions::upgrade::UpgradeAction;
use crate::actions::use_potion::UsePotionAction;
use crate::assert_matches;
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
    Remove {
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
        self.monsters.push(Monster::new(m, &mut self.rng));
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
        g.state.set_state(GameState::Victory);
        g.state.push_state(GameState::RollCombat);
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
                r.$name(&mut self.action_queue, &mut self.card_queue, play);
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
    pub monster_queue: Vec<CreatureRef>,
    pub should_add_extra_decay_status: bool,
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
            monster_queue: Default::default(),
            should_add_extra_decay_status: false,
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
        self.state.set_debug();
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

    pub fn calculate_damage(
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
        let was_bloodied = c.is_bloodied();
        let had_block = c.block != 0;
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
        if amount != 0 && c.has_status(Status::Buffer) {
            amount = 0;
            self.action_queue.push_bot(ReduceStatusAction {
                status: Status::Buffer,
                amount: 1,
                target,
            });
        }
        if target.is_player()
            && amount <= 5
            && amount >= 1
            && matches!(ty, DamageType::Attack { .. })
            && self.has_relic(RelicClass::Torii)
        {
            amount = 1;
        }
        if target.is_player() && self.has_relic(RelicClass::TungstenRod) {
            amount = (amount - 1).max(0);
        }
        if !target.is_player() && amount > 0 && amount < 5 && self.has_relic(RelicClass::Boot) {
            amount = 5;
        }
        let c = self.get_creature_mut(target);
        c.last_damage_taken = amount;
        if amount != 0 {
            c.cur_hp -= amount;
            if target.is_player() {
                self.trigger_relics_on_lose_hp();
            }
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

        if !self.get_creature(target).is_alive() {
            if !target.is_player() {
                if self.has_relic(RelicClass::GremlinHorn) {
                    self.action_queue.push_bot(GainEnergyAction(1));
                    self.action_queue.push_bot(DrawAction(1));
                }
            } else if !self.has_relic(RelicClass::MarkOfTheBloom) {
                if let Some(i) = self.potions.iter().position(|p| *p == Some(Potion::Fairy)) {
                    self.take_potion(i);
                    let percent = if self.has_relic(RelicClass::SacredBark) {
                        0.6
                    } else {
                        0.3
                    };
                    let amount = ((self.player.max_hp as f32 * percent) as i32).max(1);
                    self.player.heal(amount);
                } else if self.get_relic_value(RelicClass::LizardTail) == Some(1) {
                    self.set_relic_value(RelicClass::LizardTail, 0);
                    let amount = ((self.player.max_hp as f32 * 0.5) as i32).max(1);
                    self.player.heal(amount);
                }
            }
        }

        if !was_bloodied
            && self.get_creature(target).is_bloodied()
            && target.is_player()
            && self.has_relic(RelicClass::RedSkull)
        {
            self.action_queue.push_bot(GainStatusAction {
                status: Status::Strength,
                amount: 3,
                target: CreatureRef::player(),
            });
        }
        if self.get_creature(target).block == 0
            && had_block
            && !target.is_player()
            && self.has_relic(RelicClass::HandDrill)
        {
            self.action_queue.push_bot(GainStatusAction {
                status: Status::Vulnerable,
                amount: 2,
                target,
            });
        }

        if !self.get_creature(target).is_alive() && target.is_player() {
            self.state.push_state(GameState::Defeat);
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
            GameState::RunActions => {
                self.run_actions_until_empty();
            }
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
                self.should_add_extra_decay_status = false;

                self.setup_combat_draw_pile();

                // player pre-combat relic setup
                self.trigger_relics_at_pre_combat();

                // monster pre-combat setup
                for i in 0..self.monsters.len() {
                    if !self.monsters[i].creature.is_alive() {
                        continue;
                    }
                    self.monsters[i]
                        .behavior
                        .pre_combat(&mut self.action_queue, CreatureRef::monster(i));
                }

                self.state.set_state(GameState::PlayerTurnBegin);
                self.state.push_state(GameState::RunActions);
            }
            GameState::PlayerTurnBegin => {
                if self.combat_finished() {
                    return;
                }

                self.monsters_roll_move();

                self.num_cards_played_this_turn = 0;

                self.player
                    .start_of_turn_lose_block(self.has_relic(RelicClass::Calipers));

                if self.turn == 0 {
                    self.trigger_relics_at_combat_begin_pre_draw();
                }
                self.trigger_relics_at_turn_begin_pre_draw();
                self.player
                    .trigger_statuses_turn_begin(CreatureRef::player(), &mut self.action_queue);

                self.action_queue.push_bot(DrawAction(self.draw_per_turn));

                if self.turn == 0 {
                    self.trigger_relics_at_combat_begin_post_draw();
                }

                self.trigger_relics_at_turn_begin_post_draw();
                self.player.trigger_statuses_turn_begin_post_draw(
                    CreatureRef::player(),
                    &mut self.action_queue,
                );

                self.action_queue.push_top(StartOfTurnEnergyAction());

                self.state.set_state(GameState::PlayerTurn);
                self.state.push_state(GameState::RunActions);
            }
            GameState::PlayerTurnEnd => {
                if self.combat_finished() {
                    return;
                }
                self.should_add_extra_decay_status = true;
                self.trigger_relics_at_turn_end();
                self.player
                    .trigger_statuses_turn_end(CreatureRef::player(), &mut self.action_queue);

                self.trigger_end_of_turn_cards_in_hand();

                self.action_queue.push_bot(EndOfTurnDiscardAction());
                self.state.set_state(GameState::MonsterTurn);
                self.state.push_state(GameState::RunActions);
            }
            GameState::MonsterTurn => {
                if self.combat_finished() {
                    return;
                }
                self.monsters_pre_turn();
                for m in self.get_alive_monsters() {
                    self.monster_queue.push(m);
                }

                self.state.set_state(GameState::EndOfRound);
                self.state.push_state(GameState::RunActions);
            }
            GameState::EndOfRound => {
                if self.combat_finished() {
                    return;
                }
                self.should_add_extra_decay_status = false;
                self.monsters_end_turn();
                self.player
                    .trigger_statuses_round_end(CreatureRef::player(), &mut self.action_queue);
                for (i, m) in self.monsters.iter_mut().enumerate() {
                    m.creature.trigger_statuses_round_end(
                        CreatureRef::monster(i),
                        &mut self.action_queue,
                    );
                }
                self.state.set_state(GameState::PlayerTurnBegin);
                self.state.push_state(GameState::RunActions);
                self.turn += 1;
            }
            GameState::CombatEnd => {
                self.trigger_relics_at_combat_finish();
                self.monsters.clear();
                self.player.clear_all_status();
                self.state.set_state(GameState::RollCombat);
                self.state.push_state(GameState::RunActions);
            }
            GameState::Victory
            | GameState::Defeat
            | GameState::PlayerTurn
            | GameState::Blessing
            | GameState::TransformCard
            | GameState::RemoveCard
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
                println!("{:?}", self.state);
                unreachable!()
            }
        }
    }

    fn run(&mut self) {
        while !matches!(
            self.state.cur_state(),
            GameState::Defeat | GameState::Victory
        ) && !self.in_waiting_for_move_state()
        {
            self.run_once();
            if matches!(self.state.cur_state(), GameState::PlayerTurn) {
                self.combat_finished();
            }
        }
    }

    fn in_waiting_for_move_state(&self) -> bool {
        matches!(
            self.state.cur_state(),
            GameState::Blessing
                | GameState::PlayerTurn
                | GameState::Armaments
                | GameState::ExhaustCardsInHand { .. }
                | GameState::FetchCardFromDraw(..)
                | GameState::Memories { .. }
                | GameState::PlaceCardInHandOnTopOfDraw
                | GameState::PlaceCardInDiscardOnTopOfDraw
                | GameState::ExhaustOneCardInHand
                | GameState::ForethoughtAny { .. }
                | GameState::ForethoughtOne
                | GameState::Gamble { .. }
                | GameState::Exhume
                | GameState::DualWield(_)
                | GameState::Discovery { .. }
                | GameState::Victory
                | GameState::Defeat
        )
    }

    fn run_actions_until_empty(&mut self) {
        loop {
            if self.in_waiting_for_move_state() {
                return;
            }
            if let Some(a) = self.action_queue.pop() {
                a.run(self);
            } else if !self.card_queue.is_empty() {
                let play = self.card_queue.remove(0);
                if self.all_monsters_dead() {
                    continue;
                }
                if self.can_play_card(&play) {
                    self.action_queue.push_bot(play);
                } else if !play.is_duplicated {
                    self.action_queue.push_bot(DiscardCardAction(play.card));
                }
            } else if !self.monster_queue.is_empty() {
                let monster = self.monster_queue.remove(0);
                if !self.get_creature(monster).is_alive() {
                    continue;
                }
                self.monsters[monster.monster_index()]
                    .behavior
                    .take_turn(monster, &mut self.action_queue);
            } else {
                break;
            }
        }
        self.state.pop_state();
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

    fn monsters_end_turn(&mut self) {
        for i in 0..self.monsters.len() {
            if !self.monsters[i].creature.is_alive() {
                continue;
            }
            self.monsters[i]
                .creature
                .trigger_statuses_turn_end(CreatureRef::monster(i), &mut self.action_queue);
        }
    }

    #[cfg(test)]
    pub fn add_card_to_master_deck(&mut self, class: CardClass) {
        use crate::actions::add_card_to_master_deck::AddCardToMasterDeckAction;

        self.run_action(AddCardToMasterDeckAction(class));
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
        assert!(!self.combat_finished());
        match m {
            Move::ChooseBlessing(b) => {
                assert_matches!(self.state.cur_state(), GameState::Blessing);
                self.state.set_state(GameState::RollCombat);
                b.run(self);
            }
            Move::Transform { card_index } => {
                assert_matches!(self.state.cur_state(), GameState::TransformCard);
                let class = self.master_deck.remove(card_index).borrow().class;
                let transformed = transformed(class, &mut self.rng);
                self.action_queue
                    .push_bot(RemovedCardFromMasterDeckAction(class));
                self.action_queue
                    .push_bot(AddCardToMasterDeckAction(transformed));
                self.state.set_state(GameState::RunActions);
            }
            Move::Remove { card_index } => {
                assert_matches!(self.state.cur_state(), GameState::RemoveCard);
                let c = self.master_deck.remove(card_index);
                self.action_queue
                    .push_bot(RemovedCardFromMasterDeckAction(c.borrow().class));
                self.state.set_state(GameState::RunActions);
            }
            Move::EndTurn => {
                assert_matches!(self.state.cur_state(), GameState::PlayerTurn);
                self.state.set_state(GameState::PlayerTurnEnd);
            }
            Move::PlayCard { card_index, target } => {
                assert_matches!(self.state.cur_state(), GameState::PlayerTurn);
                let c = self.hand.remove(card_index);
                let action = PlayCardAction::new(c, target.map(CreatureRef::monster), self);
                assert!(self.can_play_card(&action));
                self.card_queue.push(action);
                self.state.push_state(GameState::RunActions);
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
                    } else {
                        return;
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
                    } else {
                        return;
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
                    } else {
                        return;
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
                    } else {
                        return;
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
                self.action_queue.push_bot(UsePotionAction {
                    potion: p,
                    target: target.map(CreatureRef::monster),
                });
                self.state.push_state(GameState::RunActions);
            }
            Move::DiscardPotion { potion_index } => {
                assert_matches!(self.state.cur_state(), GameState::PlayerTurn);
                self.take_potion(potion_index);
                return;
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
        play.free || self.energy >= play.cost
    }

    pub fn valid_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        match self.state.cur_state() {
            GameState::Blessing => {
                moves.push(Move::ChooseBlessing(Blessing::GainMaxHPSmall));
                moves.push(Move::ChooseBlessing(Blessing::CommonRelic));
                moves.push(Move::ChooseBlessing(Blessing::RemoveRelic));
                moves.push(Move::ChooseBlessing(Blessing::TransformOne));
                moves.push(Move::ChooseBlessing(Blessing::RemoveOne));
                moves.push(Move::ChooseBlessing(Blessing::RandomUncommonColorless));
                moves.push(Move::ChooseBlessing(Blessing::RandomPotion));
            }
            GameState::TransformCard => {
                for (i, c) in self.master_deck.iter().enumerate() {
                    if c.borrow().class.can_remove_from_master_deck() {
                        moves.push(Move::Transform { card_index: i });
                    }
                }
            }
            GameState::RemoveCard => {
                for (i, c) in self.master_deck.iter().enumerate() {
                    if c.borrow().class.can_remove_from_master_deck() {
                        moves.push(Move::Remove { card_index: i });
                    }
                }
            }
            GameState::PlayerTurn => {
                moves.push(Move::EndTurn);
                for (ci, c) in self.hand.iter().enumerate() {
                    if !self.can_play_card(&PlayCardAction::new(c.clone(), None, self)) {
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
                println!("{:?}", self.state);
                panic!();
            }
        }
        assert!(!moves.is_empty());
        moves
    }

    pub fn assert_no_actions(&self) {
        assert!(self.action_queue.is_empty());
        assert!(self.card_queue.is_empty());
        assert!(self.monster_queue.is_empty());
    }

    #[cfg(test)]
    pub fn run_all_actions(&mut self) {
        self.state.push_state(GameState::RunActions);
        self.run();
    }

    #[cfg(test)]
    pub fn run_action<A: Action + 'static>(&mut self, a: A) {
        self.assert_no_actions();
        self.action_queue.push_bot(a);
        self.run_all_actions();
    }

    #[cfg(test)]
    pub fn throw_potion(&mut self, potion: Potion, target: Option<CreatureRef>) {
        self.run_action(UsePotionAction { potion, target });
    }

    #[cfg(test)]
    fn play_card_impl(&mut self, card: CardRef, target: Option<CreatureRef>) {
        let action = PlayCardAction::new(card, target, self);
        assert!(self.can_play_card(&action));
        self.card_queue.push(action);
        assert_matches!(self.state.cur_state(), GameState::PlayerTurn);
        self.run_all_actions();
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
    pub fn add_cards_to_hand(&mut self, class: CardClass, amount: i32) {
        for _ in 0..amount {
            self.add_card_to_hand(class);
        }
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
    pub fn add_card_to_draw_pile_upgraded(&mut self, class: CardClass) {
        let card = self.new_card_upgraded(class);
        self.draw_pile.push(card);
    }

    #[cfg(test)]
    pub fn add_cards_to_draw_pile(&mut self, class: CardClass, amount: i32) {
        for _ in 0..amount {
            self.add_card_to_draw_pile(class);
        }
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
    pub fn get_hand_card(&self, class: CardClass) -> &CardRef {
        let cards = self
            .hand
            .iter()
            .filter(|c| c.borrow().class == class)
            .collect::<Vec<_>>();
        assert_eq!(cards.len(), 1);
        cards[0]
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

    pub fn all_monsters_dead(&self) -> bool {
        self.monsters.iter().all(|m| !m.creature.is_alive())
    }

    fn combat_finished(&mut self) -> bool {
        if self.all_monsters_dead() {
            self.state.set_state(GameState::CombatEnd);
            return true;
        }
        false
    }

    fn calculate_monster_info(&self) -> MonsterInfo {
        MonsterInfo {
            num_monsters: self.monsters.len(),
        }
    }

    pub fn heal(&mut self, cref: CreatureRef, amount: i32) {
        let c = self.get_creature_mut(cref);
        let was_bloodied = c.cur_hp <= c.max_hp / 2;
        if amount == 0 {
            return;
        }
        // check player healing relics
        c.heal(amount);
        // trigger player on heal relics
        let is_bloodied = c.cur_hp <= c.max_hp / 2;
        if was_bloodied && !is_bloodied && cref.is_player() && self.has_relic(RelicClass::RedSkull)
        {
            self.action_queue.push_bot(GainStatusAction {
                status: Status::Strength,
                amount: -3,
                target: CreatureRef::player(),
            });
        }
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
        self.assert_no_actions();

        let mut r = new_relic(class);
        r.on_equip(&mut self.action_queue);
        self.relics.push(r);
        self.state.push_state(GameState::RunActions);
        self.run();
    }
    pub fn remove_relic(&mut self, class: RelicClass) {
        let idx = self.relics.iter().position(|r| r.get_class() == class);
        let mut r = self.relics.remove(idx.unwrap());
        r.on_unequip(&mut self.action_queue);
        self.state.push_state(GameState::RunActions);
        self.run();
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
        trigger_relics_at_combat_begin_pre_draw,
        at_combat_begin_pre_draw
    );
    trigger!(
        trigger_relics_at_combat_begin_post_draw,
        at_combat_begin_post_draw
    );
    trigger!(
        trigger_relics_at_turn_begin_pre_draw,
        at_turn_begin_pre_draw
    );
    trigger!(
        trigger_relics_at_turn_begin_post_draw,
        at_turn_begin_post_draw
    );
    trigger!(trigger_relics_at_turn_end, at_turn_end);
    trigger!(trigger_relics_on_lose_hp, on_lose_hp);
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
        g.card_queue.push(PlayCardAction::new(c, None, &g));
        g.run_all_actions();
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.energy, 3);
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

    #[test]
    fn test_card_queue_after_monsters_dead() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_draw_pile(CardClass::BandageUp);
        g.add_card_to_draw_pile(CardClass::DebugKill);
        g.add_card_to_draw_pile(CardClass::BandageUp);
        let hp = g.player.cur_hp;
        g.throw_potion(Potion::Chaos, None);
        assert_eq!(g.player.cur_hp, hp + 4);
    }

    #[test]
    fn test_defeat() {
        let mut g = GameBuilder::default()
            .add_monster(AttackMonster::new(999))
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_matches!(g.result(), GameStatus::Defeat);
    }

    #[test]
    fn test_card_queue_duplicated_not_played() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Normality);
        g.throw_potion(Potion::Duplication, None);
        g.throw_potion(Potion::Duplication, None);
        g.play_card(CardClass::Thunderclap, None);
        g.play_card(CardClass::Thunderclap, None);
        assert_eq!(g.discard_pile.len(), 2);
    }
}
