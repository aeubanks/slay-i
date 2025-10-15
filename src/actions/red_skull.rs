use crate::{
    action::Action,
    actions::gain_status::GainStatusAction,
    game::{CreatureRef, Game},
    status::Status,
};

pub struct RedSkullAction();

impl Action for RedSkullAction {
    fn run(&self, game: &mut Game) {
        if game.player.is_bloodied() {
            game.action_queue.push_top(GainStatusAction {
                status: Status::Strength,
                amount: 3,
                target: CreatureRef::player(),
            });
        }
    }
}

impl std::fmt::Debug for RedSkullAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "red skull")
    }
}
