use rand::Rng;

use crate::{
    actions::{damage::DamageAction, gain_relic::GainRelicAction},
    game::{Game, RunActionsGameState},
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct ScrapOozeGameState {
    pub relic_chance: i32,
}

impl GameState for ScrapOozeGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(ReachStep {
            relic_chance: self.relic_chance,
        });
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ReachStep {
    relic_chance: i32,
}

impl Step for ReachStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue.push_bot(DamageAction::event(5));
        if game.rng.random_range(0..100) < self.relic_chance {
            let r = game.next_relic_weighted_screenless();
            game.action_queue.push_bot(GainRelicAction(r));
        } else {
            game.state.push_state(ScrapOozeGameState {
                relic_chance: self.relic_chance + 10,
            });
        }
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        format!(
            "reach: lose 5 hp, {}% chance to gain a relic",
            self.relic_chance
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        events::Event,
        game::{AscendStep, GameBuilder},
        map::RoomType,
    };

    #[test]
    fn test_reach() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::ScrapOoze);
        g.player.max_hp = 50;
        g.player.cur_hp = 50;
        g.step_test(AscendStep::new(0, 0));
        let mut count = 0;
        while g.relics.is_empty() {
            count += 1;

            g.step(0);
            assert_eq!(g.player.cur_hp, g.player.max_hp - 5 * count);
        }
    }
}
