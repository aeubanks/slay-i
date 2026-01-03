use crate::{action::Action, game::Game};

pub struct EscapePlayerAction();

impl Action for EscapePlayerAction {
    fn run(&self, game: &mut Game) {
        game.smoke_bombed = true;
    }
}

impl std::fmt::Debug for EscapePlayerAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "escape from combat")
    }
}
