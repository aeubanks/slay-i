use crate::{
    game::Game,
    master_deck::ChooseTransformMasterGameState,
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct TransmorgrifierGameState;

impl GameState for TransmorgrifierGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        if game.has_removable_cards() {
            steps.push(TransformStep);
        }
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TransformStep;

impl Step for TransformStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.state.push_state(ChooseTransformMasterGameState {
            num_cards_remaining: 1,
            upgrade: false,
        });
    }
    fn description(&self, _: &Game) -> String {
        "transform a card".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{cards::CardClass, game::GameBuilder, master_deck::ChooseTransformMasterStep};

    use super::*;

    #[test]
    fn test_remove() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Bash, 2)
            .add_cards(CardClass::AscendersBane, 1)
            .build_with_game_state(TransmorgrifierGameState);
        g.step_test(TransformStep);
        g.step_test(ChooseTransformMasterStep {
            master_index: 0,
            num_cards_remaining: 1,
            upgrade: false,
        });
        assert_eq!(g.master_deck.len(), 3);
    }
}
