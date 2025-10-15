use crate::{action::Action, game::Game};

pub struct SetHPAllMonstersAction(pub i32);

impl Action for SetHPAllMonstersAction {
    fn run(&self, game: &mut Game) {
        for m in &mut game.monsters {
            m.creature.cur_hp = 1;
        }
    }
}

impl std::fmt::Debug for SetHPAllMonstersAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "set hp all monsters {}", self.0)
    }
}
