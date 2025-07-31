use crate::{action::Action, card::CardRef, game::Game};

pub struct DiscardCardAction(pub CardRef);

impl Action for DiscardCardAction {
    fn run(&self, game: &mut Game) {
        self.0.borrow_mut().clear_temporary();
        game.discard_pile.push(self.0.clone());
    }
}

impl std::fmt::Debug for DiscardCardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "discard {:?}", self.0.borrow())
    }
}
