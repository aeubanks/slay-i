use crate::{
    action::Action,
    actions::{gain_energy::GainEnergyAction, set_energy::SetEnergyAction},
    game::{CombatType, Game},
    relic::RelicClass,
};

pub struct StartOfTurnEnergyAction();

impl Action for StartOfTurnEnergyAction {
    fn run(&self, g: &mut Game) {
        use RelicClass::*;
        let mut amount = 3;
        for r in [
            BustedCrown,
            CoffeeDripper,
            CursedKey,
            Ectoplasm,
            FusionHammer,
            PhilosophersStone,
            Sozu,
            VelvetChoker,
            MarkOfPain,
        ] {
            if g.has_relic(r) {
                amount += 1;
            }
        }
        if g.has_relic(SlaversCollar) && matches!(g.in_combat, CombatType::Boss | CombatType::Elite)
        {
            amount += 1;
        }
        if g.has_relic(RelicClass::IceCream) {
            g.action_queue.push_top(GainEnergyAction(amount));
        } else {
            g.action_queue.push_top(SetEnergyAction(amount));
        }
    }
}

impl std::fmt::Debug for StartOfTurnEnergyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start of turn energy")
    }
}
