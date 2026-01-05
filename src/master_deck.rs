use crate::{
    actions::{
        add_card_class_to_master_deck::AddCardClassToMasterDeckAction,
        add_card_to_master_deck::AddCardToMasterDeckAction,
        removed_card_from_master_deck::RemovedCardFromMasterDeckAction, upgrade::UpgradeAction,
    },
    cards::transformed,
    game::{Game, RunActionsGameState},
    state::{GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct TransformMasterGameState;

impl GameState for TransformMasterGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut moves = Steps::default();
        for (i, c) in game.master_deck.iter().enumerate() {
            if c.borrow().can_remove_from_master_deck() {
                moves.push(TransformMasterStep { master_index: i });
            }
        }
        Some(moves)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct TransformMasterStep {
    master_index: usize,
}

impl Step for TransformMasterStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        let class = game.master_deck.remove(self.master_index).borrow().class;
        let transformed = transformed(class, &mut game.rng);
        game.action_queue
            .push_bot(RemovedCardFromMasterDeckAction(class));
        game.action_queue
            .push_bot(AddCardClassToMasterDeckAction(transformed));
        game.state.push_state(RunActionsGameState);
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "transform {:?}",
            game.master_deck[self.master_index].borrow()
        )
    }
}

#[derive(Debug)]
pub struct ChooseUpgradeMasterGameState;

impl GameState for ChooseUpgradeMasterGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut moves = Steps::default();
        for (i, c) in game.master_deck.iter().enumerate() {
            if c.borrow().can_upgrade() {
                moves.push(ChooseUpgradeMasterStep { master_index: i });
            }
        }
        Some(moves)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ChooseUpgradeMasterStep {
    pub master_index: usize,
}

impl Step for ChooseUpgradeMasterStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        let c = game.master_deck[self.master_index].clone();
        game.action_queue.push_bot(UpgradeAction(c));
        game.state.push_state(RunActionsGameState);
    }

    fn description(&self, game: &Game) -> String {
        format!("upgrade {:?}", game.master_deck[self.master_index].borrow())
    }
}

#[derive(Debug)]
pub struct ChooseRemoveFromMasterGameState;

impl GameState for ChooseRemoveFromMasterGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut moves = Steps::default();
        for (i, c) in game.master_deck.iter().enumerate() {
            if c.borrow().can_remove_from_master_deck() {
                moves.push(ChooseRemoveFromMasterStep { master_index: i });
            }
        }
        Some(moves)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ChooseRemoveFromMasterStep {
    pub master_index: usize,
}

impl Step for ChooseRemoveFromMasterStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        let c = game.master_deck.remove(self.master_index);
        game.action_queue
            .push_bot(RemovedCardFromMasterDeckAction(c.borrow().class));
        game.state.push_state(RunActionsGameState);
    }

    fn description(&self, game: &Game) -> String {
        format!("remove {:?}", game.master_deck[self.master_index].borrow())
    }
}

#[derive(Debug)]
pub struct DuplicateCardInMasterGameState;

impl GameState for DuplicateCardInMasterGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut moves = Steps::default();
        for i in 0..game.master_deck.len() {
            moves.push(DuplicateCardInMasterStep { master_index: i });
        }
        Some(moves)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct DuplicateCardInMasterStep {
    pub master_index: usize,
}

impl Step for DuplicateCardInMasterStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        let original = game.master_deck[self.master_index].clone();
        let c = game.clone_card_ref_new_id(&original);
        game.action_queue.push_bot(AddCardToMasterDeckAction(c));
        game.state.push_state(RunActionsGameState);
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "duplicate {:?}",
            game.master_deck[self.master_index].borrow()
        )
    }
}
