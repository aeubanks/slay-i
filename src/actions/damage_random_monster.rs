use crate::{action::Action, actions::damage::DamageAction, game::Game};

pub struct DamageRandomMonsterAction {
    pub amount: i32,
    pub thorns: bool,
}

impl Action for DamageRandomMonsterAction {
    fn run(&self, game: &mut Game) {
        let alive = game.get_random_alive_monster();
        if self.thorns {
            game.action_queue
                .push_top(DamageAction::thorns_no_rupture(self.amount, alive));
        } else {
            game.action_queue
                .push_top(DamageAction::from_player(self.amount, alive));
        }
    }
}

impl std::fmt::Debug for DamageRandomMonsterAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "damage random monster {}", self.amount)
    }
}
