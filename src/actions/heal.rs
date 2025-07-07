use crate::{
    action::Action,
    game::{CreatureRef, Game},
};

pub struct HealAction {
    pub target: CreatureRef,
    pub amount: i32,
}

impl Action for HealAction {
    fn run(&self, game: &mut Game) {
        let c = game.get_creature_mut(self.target);
        c.cur_hp += self.amount;
        if c.cur_hp > c.max_hp {
            c.cur_hp = c.max_hp;
        }
    }
}

impl std::fmt::Debug for HealAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "heal {} hp {:?}", self.amount, self.target)
    }
}
