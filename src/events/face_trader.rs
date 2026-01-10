use crate::{
    actions::{damage::DamageAction, gain_gold::GainGoldAction, gain_relic::GainRelicAction},
    game::{CreatureRef, Game, RunActionsGameState},
    relic::RelicClass,
    rng::rand_slice,
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct FaceTraderGameState {
    damage_amount: i32,
}

impl FaceTraderGameState {
    pub fn new(game: &Game) -> Self {
        Self {
            damage_amount: 1.max(game.player.max_hp / 10),
        }
    }
}

impl GameState for FaceTraderGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(TouchStep {
            damage_amount: self.damage_amount,
        });
        steps.push(TradeStep);
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TouchStep {
    damage_amount: i32,
}

impl Step for TouchStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue.push_bot(GainGoldAction(50));
        game.action_queue.push_bot(DamageAction::thorns_no_rupture(
            self.damage_amount,
            CreatureRef::player(),
        ));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        format!("touch: gain 50 gold, lose {} hp", self.damage_amount)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TradeStep;

impl Step for TradeStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        let r = rand_slice(
            &mut game.rng,
            &[
                RelicClass::FaceOfCleric,
                RelicClass::CultistHeadpiece,
                RelicClass::SsserpentHead,
                RelicClass::NlothsHungryFace,
                RelicClass::GremlinVisage,
            ],
        );
        game.action_queue.push_bot(GainRelicAction(r));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "gain random face relic".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_matches,
        events::Event,
        game::{AscendStep, GameBuilder},
        map::RoomType,
    };

    use super::*;

    #[test]
    fn test_touch() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::FaceTrader);
        g.player.max_hp = 10;
        g.player.cur_hp = 10;
        g.step_test(AscendStep::new(0, 0));
        // ensure value is locked in even if max_hp goes up during event
        g.player.max_hp = 20;
        g.step_test(TouchStep { damage_amount: 1 });
        assert_eq!(g.player.cur_hp, 9);
    }

    #[test]
    fn test_trade() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::FaceTrader);
        g.step_test(AscendStep::new(0, 0));
        g.step_test(TradeStep);
        assert_eq!(g.relics.len(), 1);
        assert_matches!(
            g.relics[0].get_class(),
            RelicClass::FaceOfCleric
                | RelicClass::CultistHeadpiece
                | RelicClass::SsserpentHead
                | RelicClass::NlothsHungryFace
                | RelicClass::GremlinVisage
        );
    }
}
