use crate::{
    actions::{
        add_card_class_to_master_deck::AddCardClassToMasterDeckAction, damage::DamageAction,
        decrease_max_hp::DecreaseMaxHPAction, gain_relic::GainRelicAction,
    },
    cards::CardClass,
    game::{Game, RunActionsGameState},
    relic::RelicClass,
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct GoldenIdolGameState {
    damage: i32,
    max_hp_loss: i32,
}

impl GoldenIdolGameState {
    pub fn new(game: &Game) -> Self {
        Self {
            damage: (game.player.max_hp as f32 * 0.35) as i32,
            max_hp_loss: (game.player.max_hp as f32 * 0.1) as i32,
        }
    }
}

impl GameState for GoldenIdolGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(TakeStep {
            damage: self.damage,
            max_hp_loss: self.max_hp_loss,
        });
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TakeStep {
    damage: i32,
    max_hp_loss: i32,
}

impl Step for TakeStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.state.push_state(GoldenIdolTakeGameState {
            damage: self.damage,
            max_hp_loss: self.max_hp_loss,
        });
        game.action_queue
            .push_bot(GainRelicAction(RelicClass::GoldenIdol));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "take: get golden idol".to_owned()
    }
}

#[derive(Debug)]
pub struct GoldenIdolTakeGameState {
    damage: i32,
    max_hp_loss: i32,
}

impl GameState for GoldenIdolTakeGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(OutrunStep);
        steps.push(SmashStep(self.damage));
        steps.push(HideStep(self.max_hp_loss));
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OutrunStep;

impl Step for OutrunStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue
            .push_bot(AddCardClassToMasterDeckAction(CardClass::Injury));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "outrun: get injury".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SmashStep(pub i32);

impl Step for SmashStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue.push_bot(DamageAction::event(self.0));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        format!("smash: take {} damage", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HideStep(pub i32);

impl Step for HideStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue.push_bot(DecreaseMaxHPAction(self.0));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        format!("hide: lose {} max hp", self.0)
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
    fn test_outrun() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::GoldenIdol);
        g.step_test(AscendStep::new(0, 0));
        g.step(0);
        assert_eq!(g.relics[0].get_class(), RelicClass::GoldenIdol);
        g.step(0);
        assert_eq!(g.master_deck[0].borrow().class, CardClass::Injury);
    }

    #[test]
    fn test_smash() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::GoldenIdol);
        g.player.max_hp = 50;
        g.player.cur_hp = 40;
        g.step_test(AscendStep::new(0, 0));
        g.step(0);
        assert_eq!(g.relics[0].get_class(), RelicClass::GoldenIdol);
        g.step(1);
        assert_eq!(g.player.cur_hp, 40 - 17);
    }
    #[test]
    fn test_hide() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::GoldenIdol);
        g.player.max_hp = 50;
        g.player.cur_hp = 40;
        g.step_test(AscendStep::new(0, 0));
        g.step(0);
        assert_eq!(g.relics[0].get_class(), RelicClass::GoldenIdol);
        g.step(2);
        assert_eq!(g.player.max_hp, 45);
        assert_eq!(g.player.cur_hp, 40);
    }
}
