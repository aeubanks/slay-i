use crate::{action::Action, actions::set_energy::SetEnergyAction, game::Game, relic::RelicClass};

pub struct StartOfTurnEnergyAction();

impl Action for StartOfTurnEnergyAction {
    fn run(&self, g: &mut Game) {
        use RelicClass::*;
        let amount = 3 + [
            BustedCrown,
            CoffeeDripper,
            CursedKey,
            Ectoplasm,
            FusionHammer,
            PhilosophersStone,
            Sozu,
            VelvetChoker,
            MarkOfPain,
        ]
        .into_iter()
        .filter(|r| g.has_relic(*r))
        .count() as i32;
        g.action_queue.push_top(SetEnergyAction(amount));
    }
}

impl std::fmt::Debug for StartOfTurnEnergyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start of turn energy")
    }
}
