use crate::{
    action::Action,
    game::{Game, GameState},
};

pub struct ChooseUpgradeOneCardInHandAction();

impl Action for ChooseUpgradeOneCardInHandAction {
    fn run(&self, game: &mut Game) {
        if !game.hand.is_empty() {
            game.state = GameState::Armaments;
        }
    }
}

impl std::fmt::Debug for ChooseUpgradeOneCardInHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose upgrade one card in hand")
    }
}
