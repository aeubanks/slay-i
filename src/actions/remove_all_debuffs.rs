use crate::{
    action::Action,
    actions::remove_status::RemoveStatusAction,
    game::{CreatureRef, Game},
};

pub struct RemoveAllDebuffsAction();

impl Action for RemoveAllDebuffsAction {
    fn run(&self, game: &mut Game) {
        for (&status, &amount) in game.player.all_statuses() {
            if status.is_debuff(amount) {
                game.action_queue.push_top(RemoveStatusAction {
                    status,
                    target: CreatureRef::player(),
                });
            }
        }
    }
}

impl std::fmt::Debug for RemoveAllDebuffsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "remove all debuffs")
    }
}
