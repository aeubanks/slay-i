use crate::{
    action::Action, actions::place_card_on_top_of_draw::PlaceCardOnTopOfDrawAction, game::Game,
    state::GameState,
};

pub struct ChooseCardInDiscardToPlaceOnTopOfDrawAction();

impl Action for ChooseCardInDiscardToPlaceOnTopOfDrawAction {
    fn run(&self, game: &mut Game) {
        match game.discard_pile.len() {
            0 => {}
            1 => game
                .action_queue
                .push_top(PlaceCardOnTopOfDrawAction(game.discard_pile.pop().unwrap())),
            _ => game
                .state
                .push_state(GameState::PlaceCardInDiscardOnTopOfDraw),
        }
    }
}

impl std::fmt::Debug for ChooseCardInDiscardToPlaceOnTopOfDrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "move one card in discard on top of draw")
    }
}
