use crate::{action::Action, game::Game};

pub struct IncreaseMaxHPAction(pub i32);

impl Action for IncreaseMaxHPAction {
    fn run(&self, g: &mut Game) {
        g.increase_max_hp(self.0);
    }
}

impl std::fmt::Debug for IncreaseMaxHPAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "increase max hp {}", self.0)
    }
}
