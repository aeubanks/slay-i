use crate::{
    action::Action,
    game::{Game, GameState},
};

pub struct PurityAction(pub i32);

impl Action for PurityAction {
    fn run(&self, g: &mut Game) {
        if !g.hand.is_empty() {
            g.state = GameState::Purity {
                num_cards_remaining: self.0,
                cards_remaining: (0..(g.hand.len())).collect(),
            };
        }
    }
}

impl std::fmt::Debug for PurityAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "purity {}", self.0)
    }
}
