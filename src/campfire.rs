use crate::{
    actions::heal::HealAction,
    game::{CreatureRef, Game, RunActionsGameState},
    master_deck::{ChooseRemoveFromMasterGameState, ChooseUpgradeMasterGameState},
    relic::RelicClass,
    rewards::{Rewards, RewardsGameState},
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct CampfireGameState;

impl GameState for CampfireGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        if !game.has_relic(RelicClass::CoffeeDripper) {
            steps.push(CampfireRestStep);
        }
        if !game.has_relic(RelicClass::FusionHammer)
            && game.master_deck.iter().any(|c| c.borrow().can_upgrade())
        {
            steps.push(CampfireUpgradeStep);
        }
        if game
            .get_relic_value(RelicClass::Girya)
            .is_some_and(|v| v < 3)
        {
            steps.push(CampfireLiftStep);
        }
        if game.has_relic(RelicClass::PeacePipe)
            && game
                .master_deck
                .iter()
                .any(|c| c.borrow().can_remove_from_master_deck())
        {
            steps.push(CampfireTokeStep);
        }
        if game.has_relic(RelicClass::Shovel) {
            steps.push(CampfireDigStep);
        }
        if steps.steps.is_empty() {
            steps.push(ContinueStep);
        }
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CampfireRestStep;

impl Step for CampfireRestStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        let mut amount = (game.player.max_hp as f32 * 0.3) as i32;
        if game.has_relic(RelicClass::DreamCatcher) {
            let cards = Rewards::gen_card_reward(game);
            game.rewards.add_cards(cards);
            game.state.push_state(RewardsGameState);
        }
        if game.has_relic(RelicClass::RegalPillow) {
            amount += 15;
        }
        game.action_queue.push_bot(HealAction {
            target: CreatureRef::player(),
            amount,
        });
        game.state.push_state(RunActionsGameState);
    }

    fn description(&self, _: &Game) -> String {
        "campfire rest".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CampfireUpgradeStep;

impl Step for CampfireUpgradeStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        game.state.push_state(ChooseUpgradeMasterGameState);
    }

    fn description(&self, _: &Game) -> String {
        "campfire upgrade".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CampfireLiftStep;

impl Step for CampfireLiftStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        let v = game.get_relic_value(RelicClass::Girya).unwrap();
        game.set_relic_value(RelicClass::Girya, v + 1);
    }

    fn description(&self, _: &Game) -> String {
        "lift".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CampfireTokeStep;

impl Step for CampfireTokeStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        game.state.push_state(ChooseRemoveFromMasterGameState {
            num_cards_remaining: 1,
        });
    }

    fn description(&self, _: &Game) -> String {
        "toke".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CampfireDigStep;

impl Step for CampfireDigStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        let r = game.next_relic_weighted();
        game.rewards.add_relic(r);
        game.state.push_state(RewardsGameState);
    }

    fn description(&self, _: &Game) -> String {
        "dig".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        campfire::{
            CampfireDigStep, CampfireLiftStep, CampfireRestStep, CampfireTokeStep,
            CampfireUpgradeStep,
        },
        cards::CardClass,
        game::{AscendStep, DiscardPotionStep, GameBuilder, UsePotionStep},
        map::RoomType,
        master_deck::{ChooseRemoveFromMasterStep, ChooseUpgradeMasterStep},
        potion::Potion,
        relic::RelicClass,
        rewards::{RelicRewardStep, RewardExitStep},
        state::ContinueStep,
        status::Status,
        step::Step,
    };

    #[test]
    fn test_campfire_basic() {
        let g = GameBuilder::default()
            .add_card(CardClass::Strike)
            .add_card(CardClass::Defend)
            .build_campfire();
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(CampfireRestStep) as Box<dyn Step>,
                Box::new(CampfireUpgradeStep),
            ]
        );
    }

    #[test]
    fn test_campfire_no_actions_available() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::FusionHammer)
            .add_relic(RelicClass::CoffeeDripper)
            .build_campfire();
        assert_eq!(
            g.valid_steps(),
            vec![Box::new(ContinueStep) as Box<dyn Step>,]
        );
    }

    #[test]
    fn test_campfire_upgrade() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Strike)
            .add_card(CardClass::Defend)
            .build_campfire();
        g.step_test(CampfireUpgradeStep);
        g.step_test(ChooseUpgradeMasterStep { master_index: 0 });
        assert_eq!(g.master_deck[0].borrow().upgrade_count, 1);
        assert_eq!(g.master_deck[1].borrow().upgrade_count, 0);
    }

    #[test]
    fn test_campfire_upgrade_none_to_upgrade() {
        let g = GameBuilder::default()
            .add_card_upgraded(CardClass::Strike)
            .build_campfire();
        assert_eq!(
            g.valid_steps(),
            vec![Box::new(CampfireRestStep) as Box<dyn Step>,]
        );
    }

    #[test]
    fn test_campfire_rest() {
        let mut g = GameBuilder::default().build_campfire();
        g.player.cur_hp = 10;
        g.player.max_hp = 51;
        g.step_test(CampfireRestStep);
        assert_eq!(g.player.cur_hp, 10 + 15);
    }

    #[test]
    fn test_campfire_lift() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Girya)
            .build_with_rooms(&[
                RoomType::Monster,
                RoomType::Campfire,
                RoomType::Monster,
                RoomType::Campfire,
                RoomType::Campfire,
                RoomType::Campfire,
                RoomType::Monster,
            ]);

        g.roll_noop_monsters = true;

        g.step_test(AscendStep::new(0, 0));
        assert_eq!(g.player.get_status(Status::Strength), None);
        g.play_card(CardClass::DebugKillAll, None);
        g.step_test(RewardExitStep);
        g.step_test(AscendStep::new(0, 1));
        g.step_test(CampfireLiftStep);
        g.step_test(AscendStep::new(0, 2));
        assert_eq!(g.player.get_status(Status::Strength), Some(1));
        g.play_card(CardClass::DebugKillAll, None);
        g.step_test(RewardExitStep);
        g.step_test(AscendStep::new(0, 3));
        g.step_test(CampfireLiftStep);
        g.step_test(AscendStep::new(0, 4));
        g.step_test(CampfireLiftStep);
        g.step_test(AscendStep::new(0, 5));
        assert_eq!(
            g.valid_steps(),
            vec![Box::new(CampfireRestStep) as Box<dyn Step>,]
        );
        g.step_test(CampfireRestStep);
        g.step_test(AscendStep::new(0, 6));
        assert_eq!(g.player.get_status(Status::Strength), Some(3));
    }

    #[test]
    fn test_campfire_toke() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 2)
            .add_card(CardClass::Defend)
            .add_card(CardClass::CurseOfTheBell)
            .add_relic(RelicClass::PeacePipe)
            .build_campfire();
        g.step_test(CampfireTokeStep);
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(ChooseRemoveFromMasterStep {
                    master_index: 0,
                    num_cards_remaining: 1
                }) as Box<dyn Step>,
                Box::new(ChooseRemoveFromMasterStep {
                    master_index: 1,
                    num_cards_remaining: 1
                }),
                Box::new(ChooseRemoveFromMasterStep {
                    master_index: 2,
                    num_cards_remaining: 1
                }),
            ]
        );
    }

    #[test]
    fn test_campfire_toke_none_to_toke() {
        // TODO: test bottled
        let g = GameBuilder::default()
            .add_card(CardClass::CurseOfTheBell)
            .add_relic(RelicClass::PeacePipe)
            .build_campfire();
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(CampfireRestStep) as Box<dyn Step>,
                // Box::new(CampfireUpgradeStep),
            ]
        );
    }

    #[test]
    fn test_campfire_use_potion() {
        let mut g = GameBuilder::default().build_campfire();
        g.add_potion(Potion::Fire);
        g.add_potion(Potion::Fruit);
        let max_hp = g.player.max_hp;
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(CampfireRestStep) as Box<dyn Step>,
                Box::new(UsePotionStep {
                    potion_index: 1,
                    target: None
                }),
                Box::new(DiscardPotionStep { potion_index: 0 }),
                Box::new(DiscardPotionStep { potion_index: 1 }),
            ]
        );
        g.step_test(UsePotionStep {
            potion_index: 1,
            target: None,
        });
        assert_eq!(g.player.max_hp, max_hp + 5);
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(CampfireRestStep) as Box<dyn Step>,
                Box::new(DiscardPotionStep { potion_index: 0 }),
            ]
        );
    }

    #[test]
    fn test_shovel() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Shovel)
            .build_campfire();
        g.step_test(CampfireDigStep);
        g.step_test(RelicRewardStep { relic_index: 0 });
    }
}
