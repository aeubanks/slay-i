use crate::{
    action::Action,
    game::{Game, GameState},
};

pub struct ArmamentsAction();

impl Action for ArmamentsAction {
    fn run(&self, game: &mut Game) {
        if !game.hand.is_empty() {
            game.state = GameState::Armaments;
        }
    }
}

impl std::fmt::Debug for ArmamentsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose upgrade one card in hand")
    }
}
