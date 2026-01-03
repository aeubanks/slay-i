use crate::{
    action::Action,
    game::{CombatType, Game},
};

pub struct PreservedInsectAction();

impl Action for PreservedInsectAction {
    fn run(&self, game: &mut Game) {
        if matches!(game.in_combat, CombatType::Elite) {
            for m in &mut game.monsters {
                m.creature.cur_hp = (m.creature.max_hp as f32 * 0.75) as i32;
            }
        }
    }
}

impl std::fmt::Debug for PreservedInsectAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "preserved insect")
    }
}
