use crate::{
    game::Game,
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct NoopEventGameState;

impl GameState for NoopEventGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(Continue2Step);
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Continue2Step;

impl Step for Continue2Step {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.master_deck.clear();
    }
    fn description(&self, _: &Game) -> String {
        "remove all cards".to_owned()
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
    fn test_remove() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Bash, 2)
            .add_cards(CardClass::AscendersBane, 1)
            .build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::Noop);
        g.step_test(AscendStep::new(0, 0));
        g.step_test(Continue2Step);
        assert_eq!(g.master_deck.len(), 0);
    }
}
