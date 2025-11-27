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
use crate::actions::shuffle_card_into_draw::ShuffleCardIntoDrawAction;
use crate::actions::start_of_turn_energy::StartOfTurnEnergyAction;
use crate::actions::upgrade::UpgradeAction;
use crate::actions::use_potion::UsePotionAction;
use crate::assert_matches;
use crate::blessings::Blessing;
use crate::card::{Card, CardPile, CardRef};
use crate::cards::{CardClass, CardCost, CardType, transformed};
use crate::creature::Creature;
use crate::draw_pile::DrawPile;
use crate::map::Map;
use crate::monster::{Monster, MonsterBehavior, MonsterInfo};
use crate::monsters::test::NoopMonster;
use crate::potion::Potion;
use crate::queue::ActionQueue;
use crate::relic::{Relic, RelicClass, new_relic};
use crate::rng::rand_slice;
use crate::state::{GameState, GameStateManager};
use crate::status::Status;
use crate::step::Step;

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

#[derive(Eq, PartialEq, Debug)]
struct ChooseBlessingStep(Blessing);

impl Step for ChooseBlessingStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::Blessing);
        game.state.set_state(GameState::RollCombat);
        self.0.run(game);
    }

    fn description(&self, _: &Game) -> String {
        format!("{:?}", self.0)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct TransformMasterStep {
    master_index: usize,
}

impl Step for TransformMasterStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::TransformCard);
        let class = game.master_deck.remove(self.master_index).borrow().class;
        let transformed = transformed(class, &mut game.rng);
        game.action_queue
            .push_bot(RemovedCardFromMasterDeckAction(class));
        game.action_queue
            .push_bot(AddCardToMasterDeckAction(transformed));
        game.state.set_state(GameState::RunActions);
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "transform {:?}",
            game.master_deck[self.master_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct RemoveMasterStep {
    pub master_index: usize,
}

impl Step for RemoveMasterStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::RemoveCard);
        let c = game.master_deck.remove(self.master_index);
        game.action_queue
            .push_bot(RemovedCardFromMasterDeckAction(c.borrow().class));
        game.state.set_state(GameState::RunActions);
    }

    fn description(&self, game: &Game) -> String {
        format!("remove {:?}", game.master_deck[self.master_index].borrow())
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct EndTurnStep;

impl Step for EndTurnStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::PlayerTurn);
        game.state.set_state(GameState::PlayerTurnEnd);
    }

    fn description(&self, _: &Game) -> String {
        "end turn".to_string()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct UsePotionStep {
    pub potion_index: usize,
    pub target: Option<usize>,
}

impl Step for UsePotionStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::PlayerTurn);
        let p = game.take_potion(self.potion_index);
        game.action_queue.push_bot(UsePotionAction {
            potion: p,
            target: self.target.map(CreatureRef::monster),
        });
        game.state.push_state(GameState::RunActions);
    }

    fn description(&self, game: &Game) -> String {
        let mut s = format!(
            "use potion {} ({:?})",
            self.potion_index,
            game.potions[self.potion_index].unwrap()
        );
        if let Some(t) = self.target {
            s += &format!(
                " on monster {} ({})",
                t,
                game.monster_str(CreatureRef::monster(t))
            );
        }
        s
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct DiscardPotionStep {
    pub potion_index: usize,
}

impl Step for DiscardPotionStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::PlayerTurn);
        game.take_potion(self.potion_index);
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "discard potion {} ({:?})",
            self.potion_index,
            game.potions[self.potion_index].unwrap()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct PlayCardStep {
    pub hand_index: usize,
    pub target: Option<usize>,
}

impl Step for PlayCardStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::PlayerTurn);
        let c = game.hand.remove(self.hand_index);
        let action = PlayCardAction::new(c, self.target.map(CreatureRef::monster), game);
        assert!(game.can_play_card(&action));
        game.card_queue.push(action);
        game.state.push_state(GameState::RunActions);
    }

    fn description(&self, game: &Game) -> String {
        let mut s = format!(
            "play card {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        );
        if let Some(t) = self.target {
            s += &format!(
                " on monster {} ({})",
                t,
                game.monster_str(CreatureRef::monster(t))
            );
        }
        s
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ArmamentsStep {
    pub hand_index: usize,
}

impl Step for ArmamentsStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::Armaments);
        game.action_queue
            .push_top(UpgradeAction(game.hand[self.hand_index].clone()));
        game.state.pop_state();
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "upgrade card {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct PlaceCardInHandOnTopOfDrawStep {
    pub hand_index: usize,
}

impl Step for PlaceCardInHandOnTopOfDrawStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(
            game.state.cur_state(),
            GameState::PlaceCardInHandOnTopOfDraw
        );
        game.action_queue.push_top(PlaceCardOnTopOfDrawAction(
            game.hand.remove(self.hand_index),
        ));
        game.state.pop_state();
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "place card on top of draw {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct PlaceCardInDiscardOnTopOfDrawStep {
    pub discard_index: usize,
}

impl Step for PlaceCardInDiscardOnTopOfDrawStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(
            game.state.cur_state(),
            GameState::PlaceCardInDiscardOnTopOfDraw
        );
        game.action_queue.push_top(PlaceCardOnTopOfDrawAction(
            game.discard_pile.remove(self.discard_index),
        ));
        game.state.pop_state();
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "place card on top of draw {} ({:?})",
            self.discard_index,
            game.discard_pile[self.discard_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ExhaustOneCardInHandStep {
    pub hand_index: usize,
}

impl Step for ExhaustOneCardInHandStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::ExhaustOneCardInHand);
        game.action_queue
            .push_top(ExhaustCardAction(game.hand.remove(self.hand_index)));
        game.state.pop_state();
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "exhaust {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct DualWieldStep {
    pub hand_index: usize,
}

impl Step for DualWieldStep {
    fn run(&self, game: &mut Game) {
        let amount = match game.state.cur_state() {
            GameState::DualWield(amount) => *amount,
            _ => panic!(),
        };
        game.action_queue.push_top(DualWieldAction {
            card: game.hand.remove(self.hand_index),
            amount,
            destroy_original: true,
        });
        game.state.pop_state();
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "dual wield {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ExhumeStep {
    pub exhaust_index: usize,
}

impl Step for ExhumeStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::Exhume);
        game.action_queue.push_top(PlaceCardInHandAction(
            game.exhaust_pile.remove(self.exhaust_index),
        ));
        game.state.pop_state();
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "exhaust {} ({:?})",
            self.exhaust_index,
            game.exhaust_pile[self.exhaust_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct FetchFromDrawStep {
    pub draw_index: usize,
}

impl Step for FetchFromDrawStep {
    fn run(&self, game: &mut Game) {
        assert!(matches!(
            game.state.cur_state(),
            GameState::FetchCardFromDraw(_)
        ));
        let c = game.draw_pile.take(self.draw_index);
        game.action_queue.push_top(PlaceCardInHandAction(c));
        game.state.pop_state();
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "fetch {} ({:?})",
            self.draw_index,
            game.draw_pile.get(self.draw_index).borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct MemoriesStep {
    pub discard_index: usize,
}

impl Step for MemoriesStep {
    fn run(&self, game: &mut Game) {
        match game.state.cur_state_mut() {
            GameState::Memories {
                num_cards_remaining,
            } => {
                *num_cards_remaining -= 1;
                game.chosen_cards
                    .push(game.discard_pile.remove(self.discard_index));
                if *num_cards_remaining == 0 {
                    game.memories_cards();
                }
            }
            _ => unreachable!(),
        }
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "memories {} ({:?})",
            self.discard_index,
            game.discard_pile[self.discard_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ExhaustCardsInHandStep {
    pub hand_index: usize,
}

impl Step for ExhaustCardsInHandStep {
    fn run(&self, game: &mut Game) {
        match game.state.cur_state_mut() {
            GameState::ExhaustCardsInHand {
                num_cards_remaining,
            } => {
                *num_cards_remaining -= 1;
                game.chosen_cards.push(game.hand.remove(self.hand_index));
                if *num_cards_remaining == 0 || game.hand.is_empty() {
                    game.exhaust_cards();
                }
            }
            _ => unreachable!(),
        }
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "exhaust {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ExhaustCardsInHandEndStep;

impl Step for ExhaustCardsInHandEndStep {
    fn run(&self, game: &mut Game) {
        game.exhaust_cards();
    }

    fn description(&self, _: &Game) -> String {
        "end exhaust cards".to_owned()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct GambleStep {
    pub hand_index: usize,
}

impl Step for GambleStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::Gamble);
        game.chosen_cards.push(game.hand.remove(self.hand_index));
        if game.hand.is_empty() {
            game.gamble_cards();
        }
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "gamble {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct GambleEndStep;

impl Step for GambleEndStep {
    fn run(&self, game: &mut Game) {
        game.gamble_cards();
    }

    fn description(&self, _: &Game) -> String {
        "end gamble cards".to_owned()
    }
}

#[derive(Eq, PartialEq, Debug)]
struct ForethoughtOneStep {
    hand_index: usize,
}

impl Step for ForethoughtOneStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::ForethoughtOne);
        game.action_queue
            .push_top(ForethoughtAction(game.hand.remove(self.hand_index)));
        game.state.pop_state();
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "forethought {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ForethoughtAnyStep {
    pub hand_index: usize,
}

impl Step for ForethoughtAnyStep {
    fn run(&self, game: &mut Game) {
        assert_matches!(game.state.cur_state(), GameState::ForethoughtAny);
        game.chosen_cards.push(game.hand.remove(self.hand_index));
        if game.hand.is_empty() {
            game.forethought_cards();
        }
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "forethought {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ForethoughtAnyEndStep;

impl Step for ForethoughtAnyEndStep {
    fn run(&self, game: &mut Game) {
        game.forethought_cards();
    }

    fn description(&self, _: &Game) -> String {
        "end forethought cards".to_owned()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct NilrysStep {
    pub class: CardClass,
}

impl Step for NilrysStep {
    fn run(&self, game: &mut Game) {
        match game.state.cur_state() {
            &GameState::Nilrys { .. } => {
                game.action_queue.push_top(ShuffleCardIntoDrawAction {
                    class: self.class,
                    is_free: false,
                });
                game.state.pop_state();
            }
            _ => unreachable!(),
        }
    }

    fn description(&self, _: &Game) -> String {
        format!("nilrys {:?}", self.class)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct NilrysSkipStep;

impl Step for NilrysSkipStep {
    fn run(&self, game: &mut Game) {
        match game.state.cur_state() {
            &GameState::Nilrys { .. } => {
                game.state.pop_state();
            }
            _ => unreachable!(),
        }
    }

    fn description(&self, _: &Game) -> String {
        "nilrys skip".to_owned()
    }
}
#[derive(Eq, PartialEq, Debug)]
struct DiscoveryStep {
    class: CardClass,
}

impl Step for DiscoveryStep {
    fn run(&self, game: &mut Game) {
        match game.state.cur_state() {
            &GameState::Discovery {
                amount, is_free, ..
            } => {
                game.action_queue.push_top(DiscoveryAction {
                    class: self.class,
                    amount,
                    is_free,
                });
                game.state.pop_state();
            }
            _ => unreachable!(),
        }
    }

    fn description(&self, _: &Game) -> String {
        format!("discovery {:?}", self.class)
    }
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
    pub draw_pile: DrawPile<CardRef>,
    pub hand: CardPile,
    pub discard_pile: CardPile,
    pub exhaust_pile: CardPile,
    pub cur_card: Option<CardRef>,
    pub action_queue: ActionQueue,
    pub card_queue: Vec<PlayCardAction>,
    pub monster_queue: Vec<CreatureRef>,
    pub should_add_extra_decay_status: bool,
    pub num_cards_played_this_turn: i32,
    pub num_times_took_damage: i32,
    pub combat_monsters_queue: Vec<Vec<Monster>>,
    pub rng: Rand,
    pub state: GameStateManager,
    pub chosen_cards: Vec<CardRef>,
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
            num_times_took_damage: 0,
            combat_monsters_queue: vec![monsters],
            rng,
            state: GameStateManager::new(GameState::Blessing),
            chosen_cards: Default::default(),
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
        let mut c = Card {
            class,
            upgrade_count: 0,
            cost: class.base_cost(),
            exhaust: class.base_exhausts(),
            base_increase: 0,
            id,
        };
        if class == CardClass::BloodForBlood {
            let cost = c.get_base_cost();
            c.update_cost((cost - self.num_times_took_damage).max(0));
        }
        Rc::new(RefCell::new(c))
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
        if c.has_status(Status::Intangible) {
            amount = amount.min(1);
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
                self.num_times_took_damage += 1;
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

            let update_blood_for_blood_cost = |card: &CardRef| {
                let mut c = card.borrow_mut();
                if c.class == CardClass::BloodForBlood {
                    if let CardCost::Cost { base_cost, .. } = c.cost {
                        c.update_cost(0.max(base_cost - 1));
                    } else {
                        panic!();
                    }
                }
            };
            for c in &self.hand {
                update_blood_for_blood_cost(c);
            }
            for c in &self.discard_pile {
                update_blood_for_blood_cost(c);
            }
            for c in self.draw_pile.get_all() {
                update_blood_for_blood_cost(c);
            }
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
        let mut non_innate = Vec::new();
        let mut innate = Vec::new();
        for c in &self.master_deck {
            let c = self.clone_card_ref_same_id(c);
            if c.borrow().is_innate() {
                innate.push(c);
            } else {
                non_innate.push(c);
            }
        }
        let num_innate = innate.len() as i32;
        self.draw_pile = DrawPile::new(innate, non_innate);
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
                self.state.set_state(GameState::ResetCombat);
                self.state.push_state(GameState::RunActions);
            }
            GameState::ResetCombat => {
                self.monsters.clear();
                self.player.clear_all_status();
                self.num_cards_played_this_turn = 0;
                self.num_times_took_damage = 0;
                self.energy = 0;
                self.turn = 0;
                self.clear_all_piles();
                self.state.set_state(GameState::RollCombat);
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
            | GameState::Nilrys { .. }
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
                | GameState::Nilrys { .. }
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
        while let Some(c) = self.chosen_cards.pop() {
            self.action_queue.push_top(MemoriesAction(c));
        }
        self.state.pop_state();
    }

    fn exhaust_cards(&mut self) {
        while let Some(c) = self.chosen_cards.pop() {
            self.action_queue.push_top(ExhaustCardAction(c));
        }
        self.state.pop_state();
    }

    fn gamble_cards(&mut self) {
        let count = self.chosen_cards.len() as i32;
        self.action_queue.push_top(DrawAction(count));
        while let Some(c) = self.chosen_cards.pop() {
            self.action_queue.push_top(DiscardCardAction(c));
        }
        self.state.pop_state();
    }

    fn forethought_cards(&mut self) {
        while !self.chosen_cards.is_empty() {
            self.action_queue
                .push_top(ForethoughtAction(self.chosen_cards.remove(0)));
        }
        self.state.pop_state();
    }

    #[cfg(test)]
    pub fn step_test<T: Step>(&mut self, step: T) {
        self.step(Box::new(step))
    }

    pub fn step(&mut self, step: Box<dyn Step>) {
        step.run(self);
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
                .get_all()
                .iter()
                .any(|c| c.borrow().class.ty() == CardType::Skill),
            CardClass::SecretWeapon => self
                .draw_pile
                .get_all()
                .iter()
                .any(|c| c.borrow().class.ty() == CardType::Attack),
            _ => true,
        };
        if !can_play_class {
            return false;
        }
        if self.num_cards_played_this_turn >= 6 && self.has_relic(RelicClass::VelvetChoker) {
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

    pub fn valid_steps(&self) -> Vec<Box<dyn Step>> {
        let mut moves = Vec::<Box<dyn Step>>::new();
        match self.state.cur_state() {
            GameState::Blessing => {
                moves.push(Box::new(ChooseBlessingStep(Blessing::GainMaxHPSmall)));
                moves.push(Box::new(ChooseBlessingStep(Blessing::CommonRelic)));
                moves.push(Box::new(ChooseBlessingStep(Blessing::RemoveRelic)));
                moves.push(Box::new(ChooseBlessingStep(Blessing::TransformOne)));
                moves.push(Box::new(ChooseBlessingStep(Blessing::RemoveOne)));
                moves.push(Box::new(ChooseBlessingStep(
                    Blessing::RandomUncommonColorless,
                )));
                moves.push(Box::new(ChooseBlessingStep(Blessing::RandomPotion)));
            }
            GameState::TransformCard => {
                for (i, c) in self.master_deck.iter().enumerate() {
                    if c.borrow().class.can_remove_from_master_deck() {
                        moves.push(Box::new(TransformMasterStep { master_index: i }));
                    }
                }
            }
            GameState::RemoveCard => {
                for (i, c) in self.master_deck.iter().enumerate() {
                    if c.borrow().class.can_remove_from_master_deck() {
                        moves.push(Box::new(RemoveMasterStep { master_index: i }));
                    }
                }
            }
            GameState::PlayerTurn => {
                moves.push(Box::new(EndTurnStep));
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
                            moves.push(Box::new(PlayCardStep {
                                hand_index: ci,
                                target: Some(mi),
                            }));
                        }
                    } else {
                        moves.push(Box::new(PlayCardStep {
                            hand_index: ci,
                            target: None,
                        }));
                    }
                }
                for (pi, p) in self.potions.iter().enumerate() {
                    if let Some(p) = p {
                        moves.push(Box::new(DiscardPotionStep { potion_index: pi }));
                        if !p.can_use() {
                            continue;
                        }
                        if p.has_target() {
                            for (mi, m) in self.monsters.iter().enumerate() {
                                if !m.creature.is_alive() {
                                    continue;
                                }
                                moves.push(Box::new(UsePotionStep {
                                    potion_index: pi,
                                    target: Some(mi),
                                }));
                            }
                        } else {
                            moves.push(Box::new(UsePotionStep {
                                potion_index: pi,
                                target: None,
                            }));
                        }
                    }
                }
            }
            GameState::Armaments => {
                for (i, c) in self.hand.iter().enumerate() {
                    if c.borrow().can_upgrade() {
                        moves.push(Box::new(ArmamentsStep { hand_index: i }));
                    }
                }
            }
            GameState::ForethoughtOne => {
                for i in 0..self.hand.len() {
                    moves.push(Box::new(ForethoughtOneStep { hand_index: i }));
                }
            }
            GameState::ForethoughtAny { .. } => {
                moves.push(Box::new(ForethoughtAnyEndStep));
                for c in 0..self.hand.len() {
                    moves.push(Box::new(ForethoughtAnyStep { hand_index: c }));
                }
            }
            GameState::PlaceCardInHandOnTopOfDraw => {
                for i in 0..self.hand.len() {
                    moves.push(Box::new(PlaceCardInHandOnTopOfDrawStep { hand_index: i }));
                }
            }
            GameState::PlaceCardInDiscardOnTopOfDraw => {
                for i in 0..self.discard_pile.len() {
                    moves.push(Box::new(PlaceCardInDiscardOnTopOfDrawStep {
                        discard_index: i,
                    }));
                }
            }
            GameState::ExhaustOneCardInHand => {
                for i in 0..self.hand.len() {
                    moves.push(Box::new(ExhaustOneCardInHandStep { hand_index: i }));
                }
            }
            GameState::Exhume => {
                for (i, c) in self.exhaust_pile.iter().enumerate() {
                    if c.borrow().class != CardClass::Exhume {
                        moves.push(Box::new(ExhumeStep { exhaust_index: i }));
                    }
                }
            }
            GameState::DualWield(_) => {
                for (i, c) in self.hand.iter().enumerate() {
                    if can_dual_wield(c) {
                        moves.push(Box::new(DualWieldStep { hand_index: i }));
                    }
                }
            }
            GameState::FetchCardFromDraw(ty) => {
                for (i, c) in self.draw_pile.get_all().iter().enumerate() {
                    if c.borrow().class.ty() == *ty {
                        moves.push(Box::new(FetchFromDrawStep { draw_index: i }));
                    }
                }
            }
            GameState::ExhaustCardsInHand { .. } => {
                moves.push(Box::new(ExhaustCardsInHandEndStep));
                for c in 0..self.hand.len() {
                    moves.push(Box::new(ExhaustCardsInHandStep { hand_index: c }));
                }
            }
            GameState::Memories { .. } => {
                for c in 0..self.discard_pile.len() {
                    moves.push(Box::new(MemoriesStep { discard_index: c }));
                }
            }
            GameState::Gamble { .. } => {
                moves.push(Box::new(GambleEndStep));
                for c in 0..self.hand.len() {
                    moves.push(Box::new(GambleStep { hand_index: c }));
                }
            }
            GameState::Nilrys { classes, .. } => {
                moves.push(Box::new(NilrysSkipStep));
                for &class in classes {
                    moves.push(Box::new(NilrysStep { class }))
                }
            }
            GameState::Discovery { classes, .. } => {
                for &class in classes {
                    moves.push(Box::new(DiscoveryStep { class }))
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

    #[cfg(test)]
    pub fn assert_valid_steps_contains(&self, a: &dyn Step) {
        use crate::step::step_eq;

        assert!(self.valid_steps().iter().any(|s| step_eq(s, a)));
    }

    #[cfg(test)]
    pub fn assert_valid_steps_does_not_contain(&self, a: &dyn Step) {
        use crate::step::step_eq;

        assert!(!self.valid_steps().iter().any(|s| step_eq(s, a)));
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
        self.draw_pile.push_top(card);
    }

    #[cfg(test)]
    pub fn add_card_to_draw_pile_upgraded(&mut self, class: CardClass) {
        let card = self.new_card_upgraded(class);
        self.draw_pile.push_top(card);
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

    pub fn monster_str(&self, c: CreatureRef) -> String {
        let mut i = self.monsters[c.monster_index()].behavior.get_intent();
        i.modify_damage(c, self);
        format!("{}, intent: {:?}", self.get_creature(c).str(), i)
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
        if self.has_relic(RelicClass::Sozu) {
            return;
        }
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
    use crate::{
        actions::block::BlockAction, monsters::test::AttackMonster, potion::Potion, step::step_eq,
    };

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
        let valid = g.valid_steps();
        for s in [
            &EndTurnStep,
            &PlayCardStep {
                hand_index: 0,
                target: Some(0),
            },
            &PlayCardStep {
                hand_index: 0,
                target: Some(1),
            },
            &PlayCardStep {
                hand_index: 1,
                target: None,
            },
            &UsePotionStep {
                potion_index: 0,
                target: Some(0),
            },
            &UsePotionStep {
                potion_index: 0,
                target: Some(1),
            },
            &UsePotionStep {
                potion_index: 1,
                target: None,
            },
            &DiscardPotionStep { potion_index: 0 },
            &DiscardPotionStep { potion_index: 1 },
        ] as [&dyn Step; _]
        {
            assert!(valid.iter().any(|v| step_eq(v, s)));
        }
    }

    #[test]
    fn test_player_lose_block_start_of_turn() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(BlockAction::player_flat_amount(7));
        assert_eq!(g.player.block, 7);
        g.step_test(EndTurnStep);
        assert_eq!(g.player.block, 0);
    }

    #[test]
    fn test_monster_lose_block_start_of_turn() {
        let mut g = GameBuilder::default().build_combat();
        g.run_action(BlockAction::monster(CreatureRef::monster(0), 7));
        assert_eq!(g.monsters[0].creature.block, 7);
        g.step_test(EndTurnStep);
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
        g.step_test(EndTurnStep);
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
        g.step_test(PlayCardStep {
            hand_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
        g.hand.push(g.discard_pile.pop().unwrap());
        g.step_test(PlayCardStep {
            hand_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 2);

        g.step_test(EndTurnStep);
        assert_eq!(g.energy, 3);
        match &mut g.hand[0].borrow_mut().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => *free_to_play_once = true,
            _ => panic!(),
        }
        g.step_test(EndTurnStep);
        g.step_test(PlayCardStep {
            hand_index: 0,
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
        g.step_test(EndTurnStep);
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
        g.step_test(EndTurnStep);
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
