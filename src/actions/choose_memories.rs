use crate::{
    action::Action,
    actions::memories::MemoriesAction,
    game::{Game, GameState},
};

pub struct ChooseMemoriesAction(pub i32);

impl Action for ChooseMemoriesAction {
    fn run(&self, game: &mut Game) {
        if game.discard_pile.len() as i32 <= self.0 {
            let count =
                (game.discard_pile.len() as i32).min(Game::MAX_HAND_SIZE - game.hand.len() as i32);
            for _ in 0..count {
                game.action_queue
                    .push_top(MemoriesAction(game.discard_pile.remove(0)));
            }
        } else {
            game.state = GameState::Memories {
                num_cards_remaining: self.0.min(Game::MAX_HAND_SIZE - game.hand.len() as i32),
                cards_to_memories: Vec::new(),
            };
        }
    }
}

impl std::fmt::Debug for ChooseMemoriesAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose {} card(s) to memories", self.0)
    }
}
