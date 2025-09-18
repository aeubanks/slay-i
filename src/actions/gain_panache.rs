use crate::{
    action::Action,
    actions::gain_status::GainStatusAction,
    game::{CreatureRef, Game},
    status::Status,
};

pub struct GainPanacheAction {
    pub amount: i32,
}

impl Action for GainPanacheAction {
    fn run(&self, game: &mut Game) {
        let p = [
            Status::Panache4,
            Status::Panache3,
            Status::Panache2,
            Status::Panache1,
        ]
        .into_iter()
        .find(|p| game.player.has_status(*p))
        .unwrap_or(Status::Panache5);
        game.action_queue.push_top(GainStatusAction {
            status: p,
            amount: self.amount,
            target: CreatureRef::player(),
        });
    }
}

impl std::fmt::Debug for GainPanacheAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gain {} panache", self.amount)
    }
}
