use crate::{
    action::Action, actions::discard_card::DiscardCardAction, cards::CardClass, game::Game,
};

pub struct CreateCardInDiscardAction(pub CardClass);

impl Action for CreateCardInDiscardAction {
    fn run(&self, game: &mut Game) {
        let c = game.new_card(self.0);
        game.action_queue.push_top(DiscardCardAction(c));
    }
}

impl std::fmt::Debug for CreateCardInDiscardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "create {:?} in discard", self.0)
    }
}
