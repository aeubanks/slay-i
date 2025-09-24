use crate::{action::Action, game::Game, state::GameState};

pub struct ChooseForethoughtAnyAction();

impl Action for ChooseForethoughtAnyAction {
    fn run(&self, g: &mut Game) {
        if g.hand.is_empty() {
            return;
        }
        g.state.push_state(GameState::ForethoughtAny {
            cards_to_forethought: Vec::new(),
        });
    }
}

impl std::fmt::Debug for ChooseForethoughtAnyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose forethought any")
    }
}
