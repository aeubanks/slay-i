use crate::game::Game;

use dyn_eq::DynEq;
use std::fmt::Debug;

pub trait Step: DynEq + Debug {
    fn run(&self, game: &mut Game);
    fn description(&self, game: &Game) -> String;
}

dyn_eq::eq_trait_object!(Step);
