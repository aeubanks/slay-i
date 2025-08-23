use crate::{
    action::Action,
    game::{Game, GameState},
};

pub struct ChooseCardsInHandToExhaustAction(pub i32);

impl Action for ChooseCardsInHandToExhaustAction {
    fn run(&self, g: &mut Game) {
        if !g.hand.is_empty() {
            g.state = GameState::ExhaustCardsInHand {
                num_cards_remaining: self.0,
                cards_to_exhaust: Vec::new(),
            };
        }
    }
}

impl std::fmt::Debug for ChooseCardsInHandToExhaustAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "purity {}", self.0)
    }
}
