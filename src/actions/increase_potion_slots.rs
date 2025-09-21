use crate::{action::Action, game::Game};

pub struct IncreasePotionSlotsAction(pub i32);

impl Action for IncreasePotionSlotsAction {
    fn run(&self, game: &mut Game) {
        for _ in 0..self.0 {
            game.potions.push(None);
        }
    }
}

impl std::fmt::Debug for IncreasePotionSlotsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "increase potion slots {}", self.0)
    }
}
