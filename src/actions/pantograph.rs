use crate::{
    action::Action,
    actions::heal::HealAction,
    game::{CombatType, Game},
};

pub struct PantographAction();

impl Action for PantographAction {
    fn run(&self, game: &mut Game) {
        if matches!(game.in_combat, CombatType::Boss) {
            game.action_queue.push_top(HealAction::player(25));
        }
    }
}

impl std::fmt::Debug for PantographAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pantograph")
    }
}
