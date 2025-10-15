use crate::{action::Action, actions::block::BlockAction, game::Game};

pub struct OrichalcumAction(pub i32);

impl Action for OrichalcumAction {
    fn run(&self, game: &mut Game) {
        if game.player.block == 0 {
            game.action_queue
                .push_top(BlockAction::player_flat_amount(self.0));
        }
    }
}

impl std::fmt::Debug for OrichalcumAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "orichalcum {}", self.0)
    }
}
