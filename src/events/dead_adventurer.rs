use rand::RngExt;

use crate::{
    actions::{gain_gold::GainGoldAction, gain_relic::GainRelicAction},
    combat::CombatBeginGameState,
    game::{CombatType, Game, RunActionsGameState},
    monsters::Combat,
    rewards::RewardType,
    rng::remove_random,
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Reward {
    Nothing,
    Relic,
    Gold,
}

#[derive(Debug)]
pub struct DeadAdventurerGameState {
    combat: Combat,
    encounter_chance: i32,
    rewards: Vec<Reward>,
}

impl DeadAdventurerGameState {
    pub fn new(game: &mut Game) -> Self {
        let combat = match game.rng.random_range(0..3) {
            0 => Combat::GremlinNob,
            1 => Combat::ThreeSentries,
            _ => Combat::LagavulinEvent,
        };
        Self {
            combat,
            encounter_chance: 35,
            rewards: vec![Reward::Nothing, Reward::Relic, Reward::Gold],
        }
    }
}

impl GameState for DeadAdventurerGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        if !self.rewards.is_empty() {
            steps.push(SearchStep {
                combat: self.combat,
                encounter_chance: self.encounter_chance,
                rewards: self.rewards.clone(),
            });
        }
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SearchStep {
    combat: Combat,
    encounter_chance: i32,
    rewards: Vec<Reward>,
}

impl Step for SearchStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        if game.rng.random_range(0..100) < self.encounter_chance {
            game.monsters = self.combat.monsters(game);
            game.state.push_state(CombatBeginGameState(
                CombatType::Elite,
                RewardType::DeadAdventurer {
                    gold_reward: self.rewards.contains(&Reward::Gold),
                    relic_reward: self.rewards.contains(&Reward::Relic),
                },
            ));
        } else {
            let mut rewards = self.rewards.clone();
            let reward = remove_random(&mut game.rng, &mut rewards);
            game.state.push_state(DeadAdventurerGameState {
                combat: self.combat,
                encounter_chance: self.encounter_chance + 25,
                rewards,
            });
            match reward {
                Reward::Nothing => {}
                Reward::Relic => {
                    let r = game.next_relic_weighted_screenless();
                    game.action_queue.push_bot(GainRelicAction(r));
                    game.state.push_state(RunActionsGameState);
                }
                Reward::Gold => {
                    game.action_queue.push_bot(GainGoldAction(30));
                    game.state.push_state(RunActionsGameState);
                }
            }
        }
    }
    fn description(&self, _: &Game) -> String {
        format!(
            "search: {}% {:?} encounter, {:?} rewards left",
            self.encounter_chance, self.combat, self.rewards
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_not_matches,
        cards::CardClass,
        events::Event,
        game::{AscendStep, GameBuilder},
        map::RoomType,
        monster::Intent,
    };

    #[test]
    fn test_search() {
        for _ in 0..50 {
            let mut count = 0;

            let mut g = GameBuilder::default()
                .add_cards(CardClass::Bash, 2)
                .add_cards(CardClass::AscendersBane, 1)
                .build_with_rooms(&[RoomType::Event]);
            g.override_event_queue.push(Event::DeadAdventurer);
            g.step_test(AscendStep::new(0, 0));
            while g.monsters.is_empty() {
                // collected all rewards without combat
                if g.valid_steps().len() == 1 {
                    break;
                }
                g.step(0);
                count += 1;
            }
            if !g.monsters.is_empty() {
                assert!((1..=3).contains(&count));
                let name = g.monsters[0].behavior.name();
                assert!(name == "sentry" || name == "lagavulin" || name == "gremlin nob");
                assert_not_matches!(g.monsters[0].behavior.get_intent(), Intent::Sleep);
            }
        }
    }

    #[test]
    fn test_rewards() {
        let mut got_nothing = false;
        let mut got_gold = false;
        let mut got_relic = false;
        let mut got_monster = false;

        for _ in 0..100 {
            let mut g = GameBuilder::default()
                .add_cards(CardClass::Bash, 2)
                .add_cards(CardClass::AscendersBane, 1)
                .build_with_rooms(&[RoomType::Event]);
            g.override_event_queue.push(Event::DeadAdventurer);
            g.step_test(AscendStep::new(0, 0));

            g.step(0);

            if !g.monsters.is_empty() {
                got_monster = true;
                continue;
            } else if g.gold == 30 {
                got_gold = true;
            } else if !g.relics.is_empty() {
                got_relic = true;
            } else {
                got_nothing = true;
            }

            g.step(0);
            if g.monsters.is_empty() {
                assert!(g.relics.len() <= 1);
                assert!(g.gold == 0 || g.gold == 30);
            }

            if got_nothing && got_gold && got_relic && got_monster {
                break;
            }
        }

        assert!(got_nothing);
        assert!(got_gold);
        assert!(got_relic);
        assert!(got_monster);
    }
}
