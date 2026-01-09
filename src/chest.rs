use rand::Rng;

use crate::{
    actions::add_card_class_to_master_deck::AddCardClassToMasterDeckAction,
    cards::random_curse,
    game::{Game, RunActionsGameState},
    relic::{RelicClass, RelicRarity},
    rewards::RewardsGameState,
    state::{GameState, Steps},
    step::Step,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChestSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
pub struct ClosedChestGameState;

impl GameState for ClosedChestGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(OpenChestStep);
        steps.push(SkipChestStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OpenChestStep;

impl Step for OpenChestStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        if game
            .get_relic_value(RelicClass::Matryoshka)
            .is_some_and(|v| v > 0)
        {
            let rarity = if game.rng.random_range(0..4) == 0 {
                RelicRarity::Uncommon
            } else {
                RelicRarity::Common
            };
            let r = game.next_relic(rarity);
            game.rewards.add_relic(r);
            game.set_relic_value(
                RelicClass::Matryoshka,
                game.get_relic_value(RelicClass::Matryoshka).unwrap() - 1,
            );
        }

        let (common_chance, uncommon_chance, gold_chance, gold_amount) =
            match game.chest_size.unwrap() {
                ChestSize::Small => (75, 25, 50, 25),
                ChestSize::Medium => (35, 50, 35, 50),
                ChestSize::Large => (0, 75, 50, 75),
            };
        let rng = game.rng.random_range(0..100);
        let rarity = if rng < common_chance {
            RelicRarity::Common
        } else if rng < common_chance + uncommon_chance {
            RelicRarity::Uncommon
        } else {
            RelicRarity::Rare
        };
        let r = game.next_relic(rarity);
        game.rewards.add_relic(r);

        if game.rng.random_range(0..100) < gold_chance {
            let amount = (gold_amount as f32 * game.rng.random_range(0.9..=1.1)).round() as i32;
            let has_golden_idol = game.has_relic(RelicClass::GoldenIdol);
            game.rewards.add_gold(amount, has_golden_idol);
        }
        game.state.push_state(RewardsGameState);
        game.chest_size = None;

        if game.has_relic(RelicClass::CursedKey) {
            let c = random_curse(&mut game.rng);
            game.action_queue
                .push_bot(AddCardClassToMasterDeckAction(c));
            game.state.push_state(RunActionsGameState);
        }

        if !game.has_sapphire_key {
            game.rewards.has_sapphire_key = true;
        }
    }
    fn description(&self, _: &Game) -> String {
        "open chest".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SkipChestStep;

impl Step for SkipChestStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.chest_size = None;
    }
    fn description(&self, _: &Game) -> String {
        "skip chest".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_matches, assert_not_matches,
        cards::{CardClass, CardType},
        chest::{OpenChestStep, SkipChestStep},
        game::{AscendStep, GameBuilder},
        map::RoomType,
        rewards::RewardExitStep,
    };

    #[test]
    fn test_chest_open() {
        for _ in 0..10 {
            let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Treasure]);
            g.step_test(AscendStep::new(0, 0));
            let size = g.chest_size.unwrap();
            g.step_test(OpenChestStep);
            assert_eq!(g.rewards.relics.len(), 1);
            match size {
                ChestSize::Small => assert_ne!(g.rewards.relics[0].rarity(), RelicRarity::Rare),
                ChestSize::Medium => {}
                ChestSize::Large => assert_ne!(g.rewards.relics[0].rarity(), RelicRarity::Common),
            }
        }
    }

    #[test]
    fn test_chest_skip() {
        let mut g =
            GameBuilder::default().build_with_rooms(&[RoomType::Treasure, RoomType::Monster]);
        g.step_test(AscendStep::new(0, 0));
        g.step_test(SkipChestStep);
        g.step_test(AscendStep::new(0, 1));
    }

    #[test]
    fn test_cursed_key() {
        for _ in 0..10 {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::CursedKey)
                .build_with_rooms(&[RoomType::Treasure]);
            g.step_test(AscendStep::new(0, 0));
            assert_eq!(g.master_deck.len(), 0);
            g.step_test(OpenChestStep);
            assert_eq!(g.master_deck.len(), 1);
            assert_eq!(g.master_deck[0].borrow().class.ty(), CardType::Curse);
            assert_not_matches!(
                g.master_deck[0].borrow().class,
                CardClass::Necronomicurse | CardClass::AscendersBane | CardClass::CurseOfTheBell
            );
        }
    }

    #[test]
    fn test_matryoshka() {
        for _ in 0..10 {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::Matryoshka)
                .build_with_rooms(&[RoomType::Treasure, RoomType::Treasure, RoomType::Treasure]);

            g.step_test(AscendStep::new(0, 0));
            assert_eq!(g.get_relic_value(RelicClass::Matryoshka), Some(2));
            g.step_test(OpenChestStep);
            assert_eq!(g.rewards.relics.len(), 2);
            assert_matches!(
                g.rewards.relics[0].rarity(),
                RelicRarity::Common | RelicRarity::Uncommon
            );
            assert_eq!(g.get_relic_value(RelicClass::Matryoshka), Some(1));
            g.step_test(RewardExitStep);

            g.step_test(AscendStep::new(0, 1));
            assert_eq!(g.get_relic_value(RelicClass::Matryoshka), Some(1));
            g.step_test(OpenChestStep);
            assert_eq!(g.rewards.relics.len(), 2);
            assert_matches!(
                g.rewards.relics[0].rarity(),
                RelicRarity::Common | RelicRarity::Uncommon
            );
            assert_eq!(g.get_relic_value(RelicClass::Matryoshka), Some(0));
            g.step_test(RewardExitStep);

            g.step_test(AscendStep::new(0, 2));
            assert_eq!(g.get_relic_value(RelicClass::Matryoshka), Some(0));
            g.step_test(OpenChestStep);
            assert_eq!(g.rewards.relics.len(), 1);
        }
    }
}
