use crate::{
    action::Action,
    actions::exhaust_card::ExhaustCardAction,
    game::{Game, GameState},
};

pub struct ChooseCardInHandToExhaustAction();

impl Action for ChooseCardInHandToExhaustAction {
    fn run(&self, game: &mut Game) {
        match game.hand.len() {
            0 => {}
            1 => game
                .action_queue
                .push_top(ExhaustCardAction(game.hand.pop().unwrap())),
            _ => game.state = GameState::ExhaustCardInHand,
        }
    }
}

impl std::fmt::Debug for ChooseCardInHandToExhaustAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose card in hand to exhaust")
    }
}
