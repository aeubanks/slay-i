use crate::{action::Action, actions::damage::DamageAction, game::Game};

#[allow(dead_code)]
pub struct DamageRandomMonsterAction {
    pub amount: i32,
}

impl Action for DamageRandomMonsterAction {
    fn run(&self, game: &mut Game) {
        let alive = game.get_random_alive_monster();
        game.action_queue.push_top(DamageAction::from_player(
            self.amount,
            &game.player,
            &game.monsters[alive.monster_index()].creature,
            alive,
        ));
    }
}

impl std::fmt::Debug for DamageRandomMonsterAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "damage random monster {}", self.amount)
    }
}
