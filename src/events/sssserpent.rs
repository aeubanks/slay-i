use crate::{
    actions::{
        add_card_class_to_master_deck::AddCardClassToMasterDeckAction, gain_gold::GainGoldAction,
    },
    cards::CardClass,
    game::{Game, RunActionsGameState},
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct SssserpentGameState;

impl GameState for SssserpentGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(AgreeStep);
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AgreeStep;

impl Step for AgreeStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue
            .push_bot(AddCardClassToMasterDeckAction(CardClass::Doubt));
        game.action_queue.push_bot(GainGoldAction(150));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "gain 150 gold and doubt".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cards::CardClass,
        events::Event,
        game::{AscendStep, GameBuilder},
        map::RoomType,
    };

    use super::*;

    #[test]
    fn test_agree() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::Sssserpent);
        g.step_test(AscendStep::new(0, 0));
        g.step_test(AgreeStep);
        assert_eq!(g.master_deck.len(), 1);
        assert_eq!(g.master_deck[0].borrow().class, CardClass::Doubt);
        assert_eq!(g.gold, 150);
    }
}
