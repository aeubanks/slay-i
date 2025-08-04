use crate::{action::Action, game::Game};

pub struct SetEnergyAction(pub i32);

impl Action for SetEnergyAction {
    fn run(&self, game: &mut Game) {
        game.energy = self.0;
    }
}

impl std::fmt::Debug for SetEnergyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "set {} energy", self.0)
    }
}
