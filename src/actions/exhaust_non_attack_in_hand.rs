use crate::{
    action::Action, actions::exhaust_card::ExhaustCardAction, cards::CardType, game::Game,
};

#[allow(dead_code)]
pub struct ExhaustNonAttackInHandAction();

impl Action for ExhaustNonAttackInHandAction {
    fn run(&self, game: &mut Game) {
        let mut indexes_to_exhaust = Vec::new();
        for (i, c) in game.hand.iter().enumerate() {
            if c.borrow().class.ty() != CardType::Attack {
                indexes_to_exhaust.push(i);
            }
        }
        while let Some(i) = indexes_to_exhaust.pop() {
            game.action_queue
                .push_top(ExhaustCardAction(game.hand.remove(i)));
        }
    }
}

impl std::fmt::Debug for ExhaustNonAttackInHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exhaust non attack in hand")
    }
}
