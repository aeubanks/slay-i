use crate::{
    cards::CardType,
    game::Game,
    master_deck::RemoveChosenCardsGameState,
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct DivineFountainGameState;

impl GameState for DivineFountainGameState {
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
        for i in (0..game.master_deck.len()).rev() {
            let c = game.master_deck[i].borrow();
            if c.class.ty() == CardType::Curse && c.can_remove_from_master_deck() {
                drop(c);
                let c = game.master_deck.remove(i);
                game.chosen_cards.push(c);
            }
        }
        game.state.push_state(RemoveChosenCardsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "remove all removable curses".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{cards::CardClass, game::GameBuilder};

    use super::*;

    #[test]
    fn test_remove() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Bash, 2)
            .add_cards(CardClass::Pain, 2)
            .add_cards(CardClass::AscendersBane, 1)
            .build_with_game_state(DivineFountainGameState);
        g.step_test(PurifyStep);
        assert_eq!(g.master_deck.len(), 3);
    }
}
