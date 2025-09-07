use rand::seq::SliceRandom;

use crate::{action::Action, game::Game};

pub struct ShuffleDrawAction();

impl Action for ShuffleDrawAction {
    fn run(&self, game: &mut Game) {
        game.draw_pile.shuffle(&mut game.rng);
    }
}

impl std::fmt::Debug for ShuffleDrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shuffle draw")
    }
}
