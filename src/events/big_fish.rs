use crate::{
    actions::{
        add_card_class_to_master_deck::AddCardClassToMasterDeckAction, gain_relic::GainRelicAction,
        heal::HealAction, increase_max_hp::IncreaseMaxHPAction,
    },
    cards::CardClass,
    game::{Game, RunActionsGameState},
    state::{GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct BigFishGameState {
    heal_amount: i32,
}

impl BigFishGameState {
    pub fn new(game: &Game) -> Self {
        Self {
            heal_amount: game.player.max_hp / 3,
        }
    }
}

impl GameState for BigFishGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(BananaStep(self.heal_amount));
        steps.push(DonutStep);
        steps.push(BoxStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BananaStep(i32);

impl Step for BananaStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue.push_bot(HealAction::player(self.0));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        format!("banana: heal {}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DonutStep;

impl Step for DonutStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue.push_bot(IncreaseMaxHPAction(5));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "donut: gain 5 max hp".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BoxStep;

impl Step for BoxStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        let r = game.next_relic_weighted_screenless();
        game.action_queue.push_bot(GainRelicAction(r));
        game.action_queue
            .push_bot(AddCardClassToMasterDeckAction(CardClass::Regret));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "box: gain a relic and regret".to_owned()
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
    fn test_donut() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::BigFish);
        g.step_test(AscendStep::new(0, 0));
        let max_hp = g.player.max_hp;
        g.step_test(DonutStep);
        assert_eq!(g.player.max_hp, max_hp + 5);
    }

    #[test]
    fn test_banana() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::BigFish);
        g.player.max_hp = 10;
        g.player.cur_hp = 1;
        g.step_test(AscendStep::new(0, 0));
        // ensure value is locked in even if max_hp goes up during event
        g.player.max_hp = 20;
        g.step_test(BananaStep(3));
        assert_eq!(g.player.cur_hp, 4);
    }

    #[test]
    fn test_box() {
        for _ in 0..10 {
            let mut g = GameBuilder::default()
                .add_card(CardClass::Strike)
                .add_card(CardClass::Defend)
                .add_card(CardClass::Inflame)
                .build_with_rooms(&[RoomType::Event, RoomType::Monster]);
            g.override_event_queue.push(Event::BigFish);
            g.step_test(AscendStep::new(0, 0));
            g.step_test(BoxStep);
            // make sure we don't have a relic that pops up a screen
            g.step_test(AscendStep::new(0, 1));
        }
    }
}
