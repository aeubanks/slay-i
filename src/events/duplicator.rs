use crate::{
    game::Game,
    master_deck::ChooseDuplicateCardInMasterGameState,
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct DuplicatorGameState;

impl GameState for DuplicatorGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(DuplicateStep);
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DuplicateStep;

impl Step for DuplicateStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.state.push_state(ChooseDuplicateCardInMasterGameState);
    }
    fn description(&self, _: &Game) -> String {
        "duplicate a card".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{cards::CardClass, game::GameBuilder, master_deck::DuplicateCardInMasterStep};

    use super::*;

    #[test]
    fn test_remove() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Bash, 2)
            .add_cards(CardClass::AscendersBane, 1)
            .build_with_game_state(DuplicatorGameState);
        g.step_test(DuplicateStep);
        g.step_test(DuplicateCardInMasterStep { master_index: 2 });
        assert_eq!(g.master_deck.len(), 4);
        assert_eq!(g.master_deck[3].borrow().class, CardClass::AscendersBane);
    }
}
