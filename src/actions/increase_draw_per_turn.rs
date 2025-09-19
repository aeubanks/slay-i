use crate::{action::Action, game::Game};

pub struct IncreaseDrawPerTurnAction(pub i32);

impl Action for IncreaseDrawPerTurnAction {
    fn run(&self, game: &mut Game) {
        game.draw_per_turn += self.0;
        assert!(game.draw_per_turn > 0);
    }
}

impl std::fmt::Debug for IncreaseDrawPerTurnAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "increase draw per turn {}", self.0)
    }
}
