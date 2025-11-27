use crate::{action::Action, game::Game, state::GameState};

pub struct ChooseGambleAction();

impl Action for ChooseGambleAction {
    fn run(&self, game: &mut Game) {
        if game.hand.is_empty() {
            return;
        }
        game.state.push_state(GameState::Gamble);
    }
}

impl std::fmt::Debug for ChooseGambleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose cards to gamble")
    }
}
