use crate::{
    action::Action,
    actions::{draw::DrawAction, gain_energy::GainEnergyAction},
    game::{CreatureRef, Game},
    status::Status,
};

pub struct DropkickAction(pub CreatureRef);

impl Action for DropkickAction {
    fn run(&self, game: &mut Game) {
        if game
            .get_creature(self.0)
            .statuses
            .contains_key(&Status::Vulnerable)
        {
            game.action_queue.push_top(DrawAction(1));
            game.action_queue.push_top(GainEnergyAction(1));
        }
    }
}

impl std::fmt::Debug for DropkickAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dropkick")
    }
}
