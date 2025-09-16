use crate::{action::Action, game::Game};
use rand::seq::SliceRandom;

pub struct ShuffleDiscardOnTopOfDrawAction();

impl Action for ShuffleDiscardOnTopOfDrawAction {
    fn run(&self, game: &mut Game) {
        game.discard_pile.shuffle(&mut game.rng);
        game.draw_pile.append(&mut game.discard_pile);
        // In the actual game the shuffle relics trigger on
        // ShuffleDiscardOnTopOfDrawAction creation, but they add to the bottom
        // of the queue. Having the add to the top of the queue within
        // ShuffleDiscardOnTopOfDrawAction is close enough.
        game.player.trigger_relics_shuffle(&mut game.action_queue);
    }
}

impl std::fmt::Debug for ShuffleDiscardOnTopOfDrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "empty draw shuffle discard")
    }
}
