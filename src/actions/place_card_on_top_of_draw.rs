use crate::{action::Action, card::CardRef, game::Game};

pub struct PlaceCardOnTopOfDrawAction(pub CardRef);

impl Action for PlaceCardOnTopOfDrawAction {
    fn run(&self, game: &mut Game) {
        game.draw_pile.push(self.0.clone());
    }
}

impl std::fmt::Debug for PlaceCardOnTopOfDrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "place card on top of draw {:?}", self.0.borrow())
    }
}
