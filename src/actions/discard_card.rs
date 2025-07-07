use crate::{action::Action, card::CardRef, game::Game};

pub struct DiscardCardAction {
    pub card: CardRef,
}

impl Action for DiscardCardAction {
    fn run(&self, game: &mut Game) {
        game.discard_pile.push(self.card.clone());
    }
}

impl std::fmt::Debug for DiscardCardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "discard {:?}", self.card.borrow())
    }
}
