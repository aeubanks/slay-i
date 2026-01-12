use crate::{
    actions::{damage::DamageAction, upgrade_random_in_master::UpgradeTwoRandomInMasterAction},
    game::{Game, RunActionsGameState},
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct ShiningLightGameState {
    lose_hp_amount: i32,
}

impl ShiningLightGameState {
    pub fn new(game: &Game) -> Self {
        Self {
            lose_hp_amount: ((game.player.max_hp as f32 * 0.3).round()) as i32,
        }
    }
}

impl GameState for ShiningLightGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(EnterStep {
            lose_hp_amount: self.lose_hp_amount,
        });
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EnterStep {
    lose_hp_amount: i32,
}

impl Step for EnterStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue
            .push_bot(DamageAction::event(self.lose_hp_amount));
        game.action_queue
            .push_bot(UpgradeTwoRandomInMasterAction(None));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        format!("lose {} hp, upgrade two random cards", self.lose_hp_amount)
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

    #[test]
    fn test_enter() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Bash)
            .add_card(CardClass::Defend)
            .add_card(CardClass::AscendersBane)
            .build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::ShiningLight);
        g.player.cur_hp = 40;
        g.player.max_hp = 50;
        g.step_test(AscendStep::new(0, 0));
        g.step(0);
        assert_eq!(g.player.cur_hp, 40 - 15);
        assert_eq!(g.master_deck[0].borrow().upgrade_count, 1);
        assert_eq!(g.master_deck[1].borrow().upgrade_count, 1);
    }
}
