use crate::{action::Action, game::Game};

pub struct DecreaseMaxHPAction(pub i32);

impl Action for DecreaseMaxHPAction {
    fn run(&self, g: &mut Game) {
        g.player.creature.decrease_max_hp(self.0);
    }
}

impl std::fmt::Debug for DecreaseMaxHPAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "decrease max hp {}", self.0)
    }
}
