use crate::{action::Action, actions::exhaust_card::ExhaustCardAction, game::Game};

pub struct ExhaustHandAction();

impl Action for ExhaustHandAction {
    fn run(&self, game: &mut Game) {
        while let Some(c) = game.hand.pop() {
            game.action_queue.push_top(ExhaustCardAction(c));
        }
    }
}

impl std::fmt::Debug for ExhaustHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exhaust hand")
    }
}
