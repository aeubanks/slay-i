use crate::{action::Action, actions::discard_card::DiscardCardAction, game::Game};

pub struct DiscardHandAction();

impl Action for DiscardHandAction {
    fn run(&self, game: &mut Game) {
        while let Some(c) = game.hand.pop() {
            game.action_queue.push_top(DiscardCardAction(c));
        }
    }
}

impl std::fmt::Debug for DiscardHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "discard hand")
    }
}
