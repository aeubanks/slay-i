use crate::{action::Action, game::Game};

pub struct GainEnergyAction(pub i32);

impl Action for GainEnergyAction {
    fn run(&self, game: &mut Game) {
        game.energy += self.0;
    }
}

impl std::fmt::Debug for GainEnergyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gain {} energy", self.0)
    }
}
