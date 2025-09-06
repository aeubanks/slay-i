use crate::{action::Action, actions::block::BlockAction, cards::CardType, game::Game};

pub struct BlockPerNonAttackInHandAction(pub i32);

impl Action for BlockPerNonAttackInHandAction {
    fn run(&self, game: &mut Game) {
        let count = game
            .hand
            .iter()
            .filter(|c| c.borrow().class.ty() != CardType::Attack)
            .count();
        for _ in 0..count {
            game.action_queue.push_top(BlockAction::player_card(self.0));
        }
    }
}

impl std::fmt::Debug for BlockPerNonAttackInHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "block {} per non attack in hand", self.0)
    }
}
