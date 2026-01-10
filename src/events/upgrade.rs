use crate::{
    game::Game,
    master_deck::ChooseUpgradeMasterGameState,
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct UpgradeShrineGameState;

impl GameState for UpgradeShrineGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        if game.has_upgradable_cards() {
            steps.push(UpgradeStep);
        }
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct UpgradeStep;

impl Step for UpgradeStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.state.push_state(ChooseUpgradeMasterGameState);
    }
    fn description(&self, _: &Game) -> String {
        "upgrade a card".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{cards::CardClass, game::GameBuilder, master_deck::ChooseUpgradeMasterStep};

    use super::*;

    #[test]
    fn test_upgrade() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Bash, 2)
            .add_cards(CardClass::AscendersBane, 1)
            .build_with_game_state(UpgradeShrineGameState);
        g.step_test(UpgradeStep);
        g.step_test(ChooseUpgradeMasterStep { master_index: 0 });
        assert_eq!(g.master_deck.len(), 3);
        assert_eq!(g.master_deck[0].borrow().upgrade_count, 1);
    }
}
