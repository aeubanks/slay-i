use crate::{
    action::Action,
    game::{Game, GameState},
};

pub struct ChooseGambleAction();

impl Action for ChooseGambleAction {
    fn run(&self, game: &mut Game) {
        game.state = GameState::Gamble {
            cards_to_gamble: Vec::new(),
        };
    }
}

impl std::fmt::Debug for ChooseGambleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose cards to gamble")
    }
}
