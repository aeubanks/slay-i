use std::collections::HashMap;

#[cfg(test)]
use crate::action::Action;
use crate::actions::damage::{DamageAction, DamageType};
use crate::actions::discard_card::DiscardCardAction;
use crate::actions::draw::DrawAction;
use crate::actions::end_of_turn_discard::EndOfTurnDiscardAction;
use crate::actions::play_card::PlayCardAction;
use crate::actions::upgrade_one_card_in_hand::UpgradeOneCardInHandAction;
use crate::blessings::Blessing;
use crate::card::{Card, CardPile};
use crate::cards::{CardClass, CardCost, CardType, new_card, new_card_upgraded, transformed};
use crate::creature::Creature;
use crate::monster::{Monster, MonsterBehavior, MonsterInfo};
use crate::monsters::test::NoopMonster;
use crate::player::Player;
use crate::queue::ActionQueue;
use crate::relic::RelicClass;
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
    UsePotion {
        potion_index: usize,
        target: Option<usize>,
    },
}

pub enum GameStatus {
    Defeat,
    Victory,
    Combat,
    Armaments,
}

#[derive(PartialEq, Eq, Debug)]
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
    Defeat,
    Victory,
}

#[derive(Default)]
#[allow(unused)]
pub struct GameBuilder {
    master_deck: CardPile,
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
            self.master_deck.push(new_card(CardClass::Strike));
        }
        for _ in 0..4 {
            self.master_deck.push(new_card(CardClass::Defend));
        }
        self.master_deck.push(new_card(CardClass::Bash));
        self.master_deck.push(new_card(CardClass::AscendersBane));
        self
    }
    pub fn add_card(mut self, c: CardClass) -> Self {
        self.master_deck.push(new_card(c));
        self
    }
    pub fn add_card_upgraded(mut self, c: CardClass) -> Self {
        self.master_deck.push(new_card_upgraded(c));
        self
    }
    #[cfg(test)]
    pub fn add_cards(mut self, c: CardClass, amount: i32) -> Self {
        for _ in 0..amount {
            self.master_deck.push(new_card(c));
        }
        self
    }
    #[cfg(test)]
    pub fn add_cards_upgraded(mut self, c: CardClass, amount: i32) -> Self {
        for _ in 0..amount {
            self.master_deck.push(new_card_upgraded(c));
        }
        self
    }
    pub fn add_monster<M: MonsterBehavior + 'static>(mut self, m: M) -> Self {
        let hp = m.roll_hp(&mut self.rng);
        let name = m.name();
        self.monsters.push(Monster {
            creature: Creature {
                name,
                max_hp: hp,
                cur_hp: hp,
                block: 0,
                statuses: Default::default(),
            },

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
            m.creature.statuses = monster_statuses.clone();
        }
        g
    }
    pub fn build(mut self) -> Game {
        if self.monsters.is_empty() {
            self = self.add_monster(NoopMonster::new());
        }
        let mut g = Game::new(self.rng, self.master_deck, self.monsters);
        g.player.creature.statuses = self.player_statuses;
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
    pub action_queue: ActionQueue,
    pub rng: Rand,
    pub state: GameState,
}

impl Game {
    pub const MAX_HAND_SIZE: i32 = 10;

    fn new(rng: Rand, master_deck: CardPile, monsters: Vec<Monster>) -> Self {
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
            action_queue: Default::default(),
            rng,
            state: GameState::Blessing,
        };

        g.player.master_deck = master_deck;
        g.player.creature.cur_hp = (g.player.creature.cur_hp as f32 * 0.9) as i32;

        g
    }

    #[allow(dead_code)]
    pub fn set_debug(&mut self) {
        self.action_queue.set_debug();
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

    pub fn damage(&mut self, target: CreatureRef, mut amount: i32, ty: DamageType) {
        if !self.get_creature(target).is_alive() {
            return;
        }
        if let DamageType::Attack { source } = ty {
            let c = self.get_creature_mut(target);
            if let Some(a) = c
                .statuses
                .get(&Status::Thorns)
                .map(|v| DamageAction::thorns(*v, source))
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
        if amount != 0 {
            c.cur_hp -= amount;
            if c.cur_hp < 0 {
                c.cur_hp = 0;
            }
        }
    }

    fn setup_combat_draw_pile(&mut self) {
        self.draw_pile = self.player.master_deck.clone();
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
                    .trigger_relics_pre_combat(&mut self.action_queue);
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

                self.energy = 3;
                self.player.creature.block = 0;

                if self.turn == 0 {
                    self.player
                        .trigger_relics_combat_start_pre_draw(&mut self.action_queue);
                }

                self.action_queue.push_bot(DrawAction(self.draw_per_turn));

                if self.turn == 0 {
                    self.player
                        .trigger_relics_combat_start_post_draw(&mut self.action_queue);
                }

                self.player
                    .trigger_relics_turn_start(&mut self.action_queue);

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
            | GameState::Armaments => {
                unreachable!()
            }
        }
    }

    fn run(&mut self) {
        self.run_actions_until_empty();
        if self.finished() {
            return;
        }
        while self.state != GameState::PlayerTurn
            && self.state != GameState::Victory
            && self.state != GameState::Defeat
            && self.state != GameState::Armaments
        {
            self.run_once();
        }
    }

    fn run_actions_until_empty(&mut self) {
        while let Some(a) = self.action_queue.pop() {
            a.run(self);
            if self.state == GameState::Armaments {
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
        self.player.trigger_relics_turn_end(&mut self.action_queue);
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
            self.monsters[i].creature.block = 0;
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

    fn transform_card(&mut self, master_deck_index: usize) {
        let c = self.player.master_deck.remove(master_deck_index);
        let class = c.borrow().class;
        assert!(class.can_remove_from_master_deck());
        let transformed = transformed(class, &mut self.rng);
        self.add_card_to_master_deck(transformed);
    }

    pub fn add_card_to_master_deck(&mut self, class: CardClass) {
        self.player.master_deck.push(new_card(class));
    }

    pub fn result(&self) -> GameStatus {
        match self.state {
            GameState::Victory => GameStatus::Victory,
            GameState::Defeat => GameStatus::Defeat,
            GameState::Armaments => GameStatus::Armaments,
            _ => GameStatus::Combat,
        }
    }

    pub fn make_move(&mut self, m: Move) {
        match m {
            Move::ChooseBlessing(b) => {
                assert_eq!(self.state, GameState::Blessing);
                b.run(self);
                if self.state != GameState::BlessingTransform {
                    self.state = GameState::RollCombat;
                    self.run();
                }
            }
            Move::Transform { card_index } => {
                assert_eq!(self.state, GameState::BlessingTransform);
                self.transform_card(card_index);
                self.state = GameState::RollCombat;
                self.run();
            }
            Move::EndTurn => {
                assert_eq!(self.state, GameState::PlayerTurn);
                self.state = GameState::PlayerTurnEnd;
                self.run();
            }
            Move::PlayCard { card_index, target } => {
                assert_eq!(self.state, GameState::PlayerTurn);
                assert!(self.can_play_card(&self.hand[card_index].borrow()));
                self.action_queue.push_bot(PlayCardAction {
                    card: self.hand.remove(card_index),
                    target: target.map(CreatureRef::monster),
                });
                self.run();
            }
            Move::Armaments { card_index } => {
                assert_eq!(self.state, GameState::Armaments);
                self.action_queue
                    .push_top(UpgradeOneCardInHandAction(self.hand[card_index].clone()));
                self.state = GameState::PlayerTurn;
                self.run();
            }
            Move::UsePotion {
                potion_index,
                target,
            } => {
                assert_eq!(self.state, GameState::PlayerTurn);
                let p = self.player.take_potion(potion_index);
                p.behavior()(target.map(CreatureRef::monster), &mut self.action_queue);
                self.run();
            }
        }
    }

    fn can_play_card(&self, c: &Card) -> bool {
        match c.cost {
            CardCost::Zero => match c.class.ty() {
                CardType::Curse => self.player.has_relic(RelicClass::BlueCandle),
                CardType::Status => self.player.has_relic(RelicClass::MedicalKit),
                _ => unreachable!(),
            },
            CardCost::X => true,
            CardCost::Cost {
                base_cost,
                temporary_cost,
            } => self.energy >= temporary_cost.unwrap_or(base_cost),
        }
    }

    pub fn valid_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        match self.state {
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
                    let c = c.borrow();
                    if !self.can_play_card(&c) {
                        continue;
                    }
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

    #[cfg(test)]
    pub fn play_card(&mut self, class: CardClass, target: Option<CreatureRef>) {
        self.run_action(PlayCardAction {
            card: new_card(class),
            target,
        });
    }

    #[cfg(test)]
    pub fn play_card_upgraded(&mut self, class: CardClass, target: Option<CreatureRef>) {
        self.run_action(PlayCardAction {
            card: new_card_upgraded(class),
            target,
        });
    }

    fn finished(&mut self) -> bool {
        assert_ne!(self.state, GameState::Defeat);
        assert_ne!(self.state, GameState::Victory);
        if !self.player.creature.is_alive() {
            self.state = GameState::Defeat;
            return true;
        }
        if self.combat_monsters_queue.is_empty()
            && self.monsters.iter().all(|m| !m.creature.is_alive())
        {
            self.state = GameState::Victory;
            self.player
                .trigger_relics_combat_finish(&mut self.action_queue);
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
        g.hand.push(new_card(CardClass::DebugKill));
        g.hand.push(new_card(CardClass::Defend));
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

        g.run_action(DamageAction::thorns(9999, CreatureRef::monster(0)));
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
}
