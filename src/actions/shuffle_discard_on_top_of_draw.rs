use crate::{action::Action, game::Game};
use rand::seq::SliceRandom;

pub struct ShuffleDiscardOnTopOfDrawAction();

impl Action for ShuffleDiscardOnTopOfDrawAction {
    fn run(&self, game: &mut Game) {
        game.discard_pile.shuffle(&mut game.rng);
        game.draw_pile.append(&mut game.discard_pile);
    }
}

impl std::fmt::Debug for ShuffleDiscardOnTopOfDrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "empty draw shuffle discard")
    }
}
