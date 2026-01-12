use crate::{
    game::Game,
    master_deck::{
        ChooseRemoveFromMasterGameState, ChooseTransformMasterGameState,
        ChooseUpgradeMasterGameState,
    },
    state::{GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct LivingWallGameState;

impl GameState for LivingWallGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(ForgetStep);
        steps.push(ChangeStep);
        steps.push(GrowStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ForgetStep;

impl Step for ForgetStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.state.push_state(ChooseRemoveFromMasterGameState {
            num_cards_remaining: 1,
        });
    }
    fn description(&self, _: &Game) -> String {
        "remove one card".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ChangeStep;

impl Step for ChangeStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.state.push_state(ChooseTransformMasterGameState {
            num_cards_remaining: 1,
            upgrade: false,
        });
    }
    fn description(&self, _: &Game) -> String {
        "transform one card".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GrowStep;

impl Step for GrowStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.state.push_state(ChooseUpgradeMasterGameState);
    }
    fn description(&self, _: &Game) -> String {
        "upgrade one card".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cards::CardClass,
        events::Event,
        game::{AscendStep, GameBuilder},
        map::RoomType,
        master_deck::{
            ChooseRemoveFromMasterStep, ChooseTransformMasterStep, ChooseUpgradeMasterStep,
        },
    };

    

    #[test]
    fn test_forget() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Bash)
            .add_card(CardClass::Defend)
            .add_cards(CardClass::AscendersBane, 1)
            .build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::LivingWall);
        g.step_test(AscendStep::new(0, 0));
        g.step(0);
        g.step_test(ChooseRemoveFromMasterStep {
            master_index: 0,
            num_cards_remaining: 1,
        });
    }

    #[test]
    fn test_change() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Bash)
            .add_card(CardClass::Defend)
            .add_cards(CardClass::AscendersBane, 1)
            .build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::LivingWall);
        g.step_test(AscendStep::new(0, 0));
        g.step(1);
        g.step_test(ChooseTransformMasterStep {
            master_index: 0,
            num_cards_remaining: 1,
            upgrade: false,
        });
    }

    #[test]
    fn test_grow() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Bash)
            .add_card(CardClass::Defend)
            .add_cards(CardClass::AscendersBane, 1)
            .build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::LivingWall);
        g.step_test(AscendStep::new(0, 0));
        g.step(2);
        g.step_test(ChooseUpgradeMasterStep { master_index: 0 });
    }
}
