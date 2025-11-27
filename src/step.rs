use crate::game::Game;

use dyn_eq::DynEq;
use std::fmt::Debug;

pub trait Step: DynEq + Debug {
    fn run(&self, game: &mut Game);
    fn description(&self, game: &Game) -> String;
}

dyn_eq::eq_trait_object!(Step);

#[cfg(test)]
pub fn step_eq(a: &Box<dyn Step>, b: &dyn Step) -> bool {
    **a == *b
}
