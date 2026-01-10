use crate::{
    actions::damage::DamageAction,
    game::{CreatureRef, Game, RunActionsGameState},
    potion::random_potion_weighted,
    rewards::RewardsGameState,
    state::{GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct WomanInBlueGameState;

impl GameState for WomanInBlueGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(BuyOnePotionStep);
        steps.push(BuyTwoPotionsStep);
        steps.push(BuyThreePotionsStep);
        steps.push(LeaveStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BuyOnePotionStep;

impl Step for BuyOnePotionStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.gold -= 20;
        let p = random_potion_weighted(&mut game.rng);
        game.rewards.add_potion(p);
        game.state.push_state(RewardsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "buy one potion for 20 gold".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BuyTwoPotionsStep;

impl Step for BuyTwoPotionsStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.gold -= 30;
        for _ in 0..2 {
            let p = random_potion_weighted(&mut game.rng);
            game.rewards.add_potion(p);
        }
        game.state.push_state(RewardsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "buy two potions for 30 gold".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BuyThreePotionsStep;

impl Step for BuyThreePotionsStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.gold -= 40;
        for _ in 0..3 {
            let p = random_potion_weighted(&mut game.rng);
            game.rewards.add_potion(p);
        }
        game.state.push_state(RewardsGameState);
    }
    fn description(&self, _: &Game) -> String {
        "buy three potions for 40 gold".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LeaveStep;

fn damage_amount(game: &Game) -> i32 {
    (game.player.max_hp as f32 * 0.05).ceil() as i32
}

impl Step for LeaveStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.action_queue.push_bot(DamageAction::thorns_no_rupture(
            damage_amount(game),
            CreatureRef::player(),
        ));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, game: &Game) -> String {
        format!("take {} damage", damage_amount(game))
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
    fn test_buy_one_potion() {
        let mut g = GameBuilder::default().build_with_game_state(WomanInBlueGameState);
        g.gold = 50;
        g.step_test(BuyOnePotionStep);
        assert_eq!(g.gold, 30);
        assert_eq!(g.rewards.potions.len(), 1);
    }

    #[test]
    fn test_buy_two_potions() {
        let mut g = GameBuilder::default().build_with_game_state(WomanInBlueGameState);
        g.gold = 50;
        g.step_test(BuyTwoPotionsStep);
        assert_eq!(g.gold, 20);
        assert_eq!(g.rewards.potions.len(), 2);
    }

    #[test]
    fn test_buy_three_potions() {
        let mut g = GameBuilder::default().build_with_game_state(WomanInBlueGameState);
        g.gold = 50;
        g.step_test(BuyThreePotionsStep);
        assert_eq!(g.gold, 10);
        assert_eq!(g.rewards.potions.len(), 3);
    }

    #[test]
    fn test_leave() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::WomanInBlue);
        g.step_test(AscendStep::new(0, 0));
        g.player.cur_hp = 10;
        g.player.max_hp = 21;
        g.gold = 50;
        g.step_test(LeaveStep);
        assert_eq!(g.gold, 50);
        assert_eq!(g.player.cur_hp, 10 - 2);
    }
}
