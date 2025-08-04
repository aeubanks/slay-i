use crate::{action::Action, game::Game};

pub struct GainEnergyAction(pub i32);

impl Action for GainEnergyAction {
    fn run(&self, game: &mut Game) {
        game.energy += self.0;
        game.energy = game.energy.clamp(0, 999);
    }
}

impl std::fmt::Debug for GainEnergyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gain {} energy", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{actions::gain_energy::GainEnergyAction, game::GameBuilder};

    #[test]
    fn test_bounds() {
        let mut g = GameBuilder::default().build_combat();
        g.run_action(GainEnergyAction(-10));
        assert_eq!(g.energy, 0);
        g.run_action(GainEnergyAction(1000));
        assert_eq!(g.energy, 999);
    }
}
