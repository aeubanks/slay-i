use crate::{action::Action, actions::discard_card::DiscardCardAction, card::CardRef, game::Game};

pub struct PlaceCardInHandAction(pub CardRef);

impl Action for PlaceCardInHandAction {
    fn run(&self, game: &mut Game) {
        if game.hand_is_full() {
            game.action_queue
                .push_top(DiscardCardAction(self.0.clone()));
        } else {
            game.hand.push(self.0.clone());
        }
    }
}

impl std::fmt::Debug for PlaceCardInHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "place card in hand {:?}", self.0.borrow())
    }
}
