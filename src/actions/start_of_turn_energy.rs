use crate::{action::Action, actions::set_energy::SetEnergyAction, game::Game};

pub struct StartOfTurnEnergyAction();

impl Action for StartOfTurnEnergyAction {
    fn run(&self, g: &mut Game) {
        g.action_queue.push_top(SetEnergyAction(3));
    }
}

impl std::fmt::Debug for StartOfTurnEnergyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start of turn energy")
    }
}
