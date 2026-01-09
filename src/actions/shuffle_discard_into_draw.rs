use crate::{action::Action, game::Game};

pub struct ShuffleDiscardIntoDrawAction();

impl Action for ShuffleDiscardIntoDrawAction {
    fn run(&self, game: &mut Game) {
        let mut discard = Vec::new();
        std::mem::swap(&mut game.discard_pile, &mut discard);
        for c in discard {
            game.draw_pile.push_top(c);
        }
        game.draw_pile.shuffle_all(&mut game.rng);

        // In the actual game the shuffle relics trigger on
        // ShuffleDiscardOnTopOfDrawAction creation, but they add to the bottom
        // of the queue. Having the add to the top of the queue within
        // ShuffleDiscardOnTopOfDrawAction is close enough.
        game.trigger_relics_on_shuffle();
    }
}

impl std::fmt::Debug for ShuffleDiscardIntoDrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "empty draw shuffle discard")
    }
}
