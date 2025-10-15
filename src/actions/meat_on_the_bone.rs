use crate::{
    action::Action,
    actions::heal::HealAction,
    game::{CreatureRef, Game},
};

pub struct MeatOnTheBoneAction(pub i32);

impl Action for MeatOnTheBoneAction {
    fn run(&self, game: &mut Game) {
        if game.player.cur_hp <= game.player.max_hp / 2 {
            game.action_queue.push_top(HealAction {
                target: CreatureRef::player(),
                amount: self.0,
            });
        }
    }
}

impl std::fmt::Debug for MeatOnTheBoneAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "meat on the bone {}", self.0)
    }
}
