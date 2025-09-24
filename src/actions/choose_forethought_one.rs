use crate::{
    action::Action, actions::forethought::ForethoughtAction, game::Game, state::GameState,
};

pub struct ChooseForethoughtOneAction();

impl Action for ChooseForethoughtOneAction {
    fn run(&self, game: &mut Game) {
        match game.hand.len() {
            0 => {}
            1 => game
                .action_queue
                .push_top(ForethoughtAction(game.hand.pop().unwrap())),
            _ => game.state.push_state(GameState::ForethoughtOne),
        }
    }
}

impl std::fmt::Debug for ChooseForethoughtOneAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose forethought one")
    }
}
