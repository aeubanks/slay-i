use crate::{action::Action, game::Game};

pub struct DiscardHandAction();

impl Action for DiscardHandAction {
    fn run(&self, game: &mut Game) {
        game.discard_pile.append(&mut game.hand);
    }
}

impl std::fmt::Debug for DiscardHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "discard hand")
    }
}
