use std::fmt::Debug;

use crate::game::Game;

pub trait Action: Debug {
    fn run(&self, game: &mut Game);
}
