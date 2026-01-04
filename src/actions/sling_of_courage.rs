use crate::{
    action::Action,
    actions::gain_status::GainStatusAction,
    game::{CombatType, CreatureRef, Game},
    status::Status,
};

pub struct SlingOfCourageAction();

impl Action for SlingOfCourageAction {
    fn run(&self, game: &mut Game) {
        if matches!(game.in_combat, CombatType::Elite) {
            game.action_queue.push_top(GainStatusAction {
                status: Status::Strength,
                amount: 2,
                target: CreatureRef::player(),
            });
        }
    }
}

impl std::fmt::Debug for SlingOfCourageAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sling of courage")
    }
}
