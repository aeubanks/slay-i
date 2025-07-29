use crate::{action::Action, card::CardRef, game::Game};

pub struct ExhaustCardAction {
    pub card: CardRef,
}

impl Action for ExhaustCardAction {
    fn run(&self, game: &mut Game) {
        self.card.borrow_mut().clear_temporary();
        game.exhaust_pile.push(self.card.clone());
    }
}

impl std::fmt::Debug for ExhaustCardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exhaust {:?}", self.card)
    }
}
