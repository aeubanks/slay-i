use crate::{action::Action, actions::play_top_card::PlayTopCardAction, game::Game};

pub struct MayhemAction();

impl Action for MayhemAction {
    fn run(&self, game: &mut Game) {
        game.action_queue.push_bot(PlayTopCardAction {
            force_exhaust: false,
        });
    }
}

impl std::fmt::Debug for MayhemAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mayhem")
    }
}
