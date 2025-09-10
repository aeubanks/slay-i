use rand::Rng;

use crate::{action::Action, cards::CardCost, game::Game};

pub struct RandomizeHandCostAction();

impl Action for RandomizeHandCostAction {
    fn run(&self, game: &mut Game) {
        for c in &game.hand {
            if let CardCost::Cost {
                base_cost,
                temporary_cost,
                ..
            } = &mut c.borrow_mut().cost
            {
                *base_cost = game.rng.random_range(0..=3);
                *temporary_cost = None;
            }
        }
    }
}

impl std::fmt::Debug for RandomizeHandCostAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "randomize hand cost")
    }
}
