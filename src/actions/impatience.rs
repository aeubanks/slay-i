use crate::{action::Action, actions::draw::DrawAction, cards::CardType, game::Game};

pub struct ImpatienceAction(pub i32);

impl Action for ImpatienceAction {
    fn run(&self, game: &mut Game) {
        if game
            .hand
            .iter()
            .all(|c| c.borrow().class.ty() != CardType::Attack)
        {
            game.action_queue.push_top(DrawAction(self.0));
        }
    }
}

impl std::fmt::Debug for ImpatienceAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "impatience {}", self.0)
    }
}
