use crate::{
    action::Action,
    actions::gain_status::GainStatusAction,
    game::{CreatureRef, Game},
    status::Status,
};

pub struct DoubleStrengthAction();

impl Action for DoubleStrengthAction {
    fn run(&self, game: &mut Game) {
        if let Some(v) = game.player.creature.get_status(Status::Strength) {
            game.action_queue.push_top(GainStatusAction {
                status: Status::Strength,
                amount: v,
                target: CreatureRef::player(),
            });
        }
    }
}

impl std::fmt::Debug for DoubleStrengthAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "double strength")
    }
}
