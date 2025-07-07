use crate::{
    action::Action,
    game::{CreatureRef, Game},
};

pub struct LoseHPAction {
    pub target: CreatureRef,
    pub amount: i32,
}

impl Action for LoseHPAction {
    fn run(&self, game: &mut Game) {
        let c = game.get_creature_mut(self.target);
        c.cur_hp -= self.amount;
    }
}

impl std::fmt::Debug for LoseHPAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lose {} hp {:?}", self.amount, self.target)
    }
}
