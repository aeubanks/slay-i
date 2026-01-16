use crate::{
    actions::{add_card_class_to_master_deck::AddCardClassToMasterDeckAction, heal::HealAction},
    cards::CardClass,
    combat::CombatBeginGameState,
    game::{CombatType, Game, RunActionsGameState},
    monsters::Combat,
    rewards::RewardType,
    state::{GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct MushroomsGameState;

impl GameState for MushroomsGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(StompStep);
        steps.push(EatStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct StompStep;

impl Step for StompStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.monsters = Combat::ThreeFungiBeasts.monsters(game);
        game.state.push_state(CombatBeginGameState(
            CombatType::Normal,
            RewardType::Mushrooms,
        ));
    }
    fn description(&self, _: &Game) -> String {
        "fight mushrooms".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EatStep;

impl Step for EatStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue
            .push_bot(HealAction::player(game.player.max_hp / 4));
        game.action_queue
            .push_bot(AddCardClassToMasterDeckAction(CardClass::Parasite));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, game: &Game) -> String {
        format!("eat: heal {} and gain parasite", game.player.max_hp / 4)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cards::CardClass,
        events::Event,
        game::{AscendStep, GameBuilder},
        map::RoomType,
        relic::RelicClass,
    };

    #[test]
    fn test_stomp() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::Mushrooms);
        g.step_test(AscendStep::new(0, 0));
        g.step(0);
        assert_eq!(g.monsters.len(), 3);
        assert!(
            g.monsters
                .iter()
                .all(|m| m.behavior.name().contains("fungi"))
        );
        g.play_card(CardClass::DebugKillAll, None);
        assert_eq!(g.rewards.relics[0], RelicClass::OddMushroom);
        assert!(g.rewards.gold >= 20);
        assert!(g.rewards.gold <= 30);
    }

    #[test]
    fn test_eat() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::Mushrooms);
        g.player.max_hp = 60;
        g.player.cur_hp = 30;
        g.step_test(AscendStep::new(0, 0));
        g.step(1);
        assert_eq!(g.master_deck.len(), 1);
        assert_eq!(g.master_deck[0].borrow().class, CardClass::Parasite);
        assert_eq!(g.player.cur_hp, 30 + 15);
    }
}
