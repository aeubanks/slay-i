use crate::{
    game::Game,
    master_deck::ChooseRemoveFromMasterGameState,
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct PurifierGameState;

impl GameState for PurifierGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        if game.has_removable_cards() {
            steps.push(PurifyStep);
        }
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PurifyStep;

impl Step for PurifyStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.state.push_state(ChooseRemoveFromMasterGameState {
            num_cards_remaining: 1,
        });
    }
    fn description(&self, _: &Game) -> String {
        "remove a card".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{cards::CardClass, game::GameBuilder, master_deck::ChooseRemoveFromMasterStep};

    use super::*;

    #[test]
    fn test_remove() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Bash, 2)
            .add_cards(CardClass::AscendersBane, 1)
            .build_with_game_state(PurifierGameState);
        g.step_test(PurifyStep);
        g.step_test(ChooseRemoveFromMasterStep {
            master_index: 0,
            num_cards_remaining: 1,
        });
        assert_eq!(g.master_deck.len(), 2);
    }
}
