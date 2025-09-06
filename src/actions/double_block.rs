use crate::{action::Action, actions::block::BlockAction, game::Game};

pub struct DoubleBlockAction();

impl Action for DoubleBlockAction {
    fn run(&self, game: &mut Game) {
        if game.player.creature.block == 0 {
            return;
        }
        game.action_queue
            .push_top(BlockAction::player_flat_amount(game.player.creature.block));
    }
}

impl std::fmt::Debug for DoubleBlockAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "double block")
    }
}
