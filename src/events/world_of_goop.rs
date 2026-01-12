use rand::Rng;

use crate::{
    actions::{damage::DamageAction, gain_gold::GainGoldAction},
    game::{Game, RunActionsGameState},
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct WorldOfGoopGameState {
    lose_gold_amount: i32,
}

impl WorldOfGoopGameState {
    pub fn new(game: &mut Game) -> Self {
        Self {
            lose_gold_amount: game.rng.random_range(35..=75),
        }
    }
}

impl GameState for WorldOfGoopGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(GatherStep);
        steps.push(LeaveStep {
            lose_gold_amount: self.lose_gold_amount,
        });
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GatherStep;

impl Step for GatherStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue.push_bot(DamageAction::event(11));
        game.action_queue.push_bot(GainGoldAction(75));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "lose 11 hp, gain 75 gold".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LeaveStep {
    lose_gold_amount: i32,
}

impl Step for LeaveStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.gold -= self.lose_gold_amount;
    }
    fn description(&self, _: &Game) -> String {
        format!("lose {} gold", self.lose_gold_amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        events::Event,
        game::{AscendStep, GameBuilder},
        map::RoomType,
    };

    use super::*;

    #[test]
    fn test_gather() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::WorldOfGoop);
        g.player.cur_hp = 50;
        g.step_test(AscendStep::new(0, 0));
        g.step_test(GatherStep);
        assert_eq!(g.gold, 75);
        assert_eq!(g.player.cur_hp, 50 - 11);
    }

    #[test]
    fn test_leave() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::WorldOfGoop);
        g.player.cur_hp = 50;
        g.gold = 100;
        g.step_test(AscendStep::new(0, 0));
        g.step(1);
        assert!(g.gold >= 100 - 75);
        assert!(g.gold <= 100 - 35);
        assert_eq!(g.player.cur_hp, 50);
    }
}
