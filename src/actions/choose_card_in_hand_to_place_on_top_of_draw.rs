use crate::{
    action::Action,
    actions::place_card_on_top_of_draw::PlaceCardOnTopOfDrawAction,
    game::{Game, GameState},
};

pub struct ChooseCardInHandToPlaceOnTopOfDraw();

impl Action for ChooseCardInHandToPlaceOnTopOfDraw {
    fn run(&self, game: &mut Game) {
        match game.hand.len() {
            0 => {}
            1 => game
                .action_queue
                .push_top(PlaceCardOnTopOfDrawAction(game.hand.pop().unwrap())),
            _ => game.state = GameState::PlaceCardInHandOnTopOfDraw,
        }
    }
}

impl std::fmt::Debug for ChooseCardInHandToPlaceOnTopOfDraw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "move one card in hand on top of draw")
    }
}
