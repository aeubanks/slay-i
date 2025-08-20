use rand::Rng;

use crate::{action::Action, actions::exhaust_card::ExhaustCardAction, game::Game};

pub struct ExhaustRandomCardInHand();

impl Action for ExhaustRandomCardInHand {
    fn run(&self, game: &mut Game) {
        match game.hand.len() {
            0 => {}
            1 => game
                .action_queue
                .push_top(ExhaustCardAction(game.hand.pop().unwrap())),
            _ => game.action_queue.push_top(ExhaustCardAction(
                game.hand.remove(game.rng.random_range(0..game.hand.len())),
            )),
        }
    }
}

impl std::fmt::Debug for ExhaustRandomCardInHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exhaust random card in hand")
    }
}
