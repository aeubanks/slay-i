use crate::{
    actions::{add_card_to_master_deck::AddCardToMasterDeckAction, gain_relic::GainRelicAction},
    cards::CardClass,
    game::{Game, RunActionsGameState},
    master_deck::ChooseUpgradeMasterGameState,
    relic::RelicClass,
    state::{GameState, NoopStep, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct AccursedBlackSmithGameState;

impl GameState for AccursedBlackSmithGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        if game.master_deck.iter().any(|c| c.borrow().can_upgrade()) {
            steps.push(UpgradeStep);
        }
        steps.push(RummageStep);
        steps.push(NoopStep);
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

#[derive(Debug, PartialEq, Eq)]
struct RummageStep;

impl Step for RummageStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue
            .push_bot(AddCardToMasterDeckAction(CardClass::Pain));
        game.action_queue
            .push_bot(GainRelicAction(RelicClass::WarpedTongs));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "rummage (warped tongs + pain)".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{game::GameBuilder, master_deck::ChooseUpgradeMasterStep};

    use super::*;

    #[test]
    fn test_upgrade() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Anger, 2)
            .build_with_game_state(AccursedBlackSmithGameState);
        g.step_test(UpgradeStep);
        g.step_test(ChooseUpgradeMasterStep { master_index: 0 });
        assert_eq!(g.master_deck[0].borrow().upgrade_count, 1);
    }

    #[test]
    fn test_no_upgradable_cards() {
        let g = GameBuilder::default()
            .add_cards_upgraded(CardClass::Anger, 2)
            .build_with_game_state(AccursedBlackSmithGameState);
        assert_eq!(
            g.valid_steps(),
            vec![Box::new(RummageStep) as Box<dyn Step>, Box::new(NoopStep),]
        );
    }

    #[test]
    fn test_rummage() {
        let mut g = GameBuilder::default().build_with_game_state(AccursedBlackSmithGameState);
        g.step_test(RummageStep);
        assert_eq!(g.master_deck.len(), 1);
        assert_eq!(g.master_deck[0].borrow().class, CardClass::Pain);
        assert!(g.has_relic(RelicClass::WarpedTongs));
    }
}
