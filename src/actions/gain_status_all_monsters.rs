use crate::{
    action::Action,
    actions::gain_status::GainStatusAction,
    game::{CreatureRef, Game},
    status::Status,
};

pub struct GainStatusAllMonstersAction {
    pub status: Status,
    pub amount: i32,
}

impl Action for GainStatusAllMonstersAction {
    fn run(&self, game: &mut Game) {
        for (mi, m) in game.monsters.iter().enumerate() {
            if !m.creature.is_alive() {
                continue;
            }
            let target = CreatureRef::monster(mi);
            game.action_queue.push_top(GainStatusAction {
                status: self.status,
                amount: self.amount,
                target,
            });
        }
    }
}

impl std::fmt::Debug for GainStatusAllMonstersAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gain {} all monsters {:?}", self.amount, self.status)
    }
}
