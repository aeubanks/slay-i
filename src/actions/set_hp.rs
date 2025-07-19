use crate::{
    action::Action,
    game::{CreatureRef, Game},
};

pub struct SetHPAction {
    pub target: CreatureRef,
    pub hp: i32,
}

impl Action for SetHPAction {
    fn run(&self, game: &mut Game) {
        let c = game.get_creature_mut(self.target);
        c.cur_hp = self.hp;
    }
}

impl std::fmt::Debug for SetHPAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "set {} hp {:?}", self.hp, self.target)
    }
}
