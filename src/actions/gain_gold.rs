use crate::{action::Action, game::Game};

pub struct GainGoldAction(pub i32);

impl Action for GainGoldAction {
    fn run(&self, game: &mut Game) {
        game.player.gold += self.0;
    }
}

impl std::fmt::Debug for GainGoldAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gain gold {}", self.0)
    }
}
