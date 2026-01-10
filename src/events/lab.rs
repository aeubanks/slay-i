use crate::{
    game::Game, potion::random_potion_weighted, rewards::RewardsGameState, state::GameState,
};

#[derive(Debug)]
pub struct LabGameState;

impl GameState for LabGameState {
    fn run(&self, game: &mut Game) {
        for _ in 0..2 {
            let p = random_potion_weighted(&mut game.rng);
            game.rewards.add_potion(p);
        }
        game.state.push_state(RewardsGameState);
    }
}

#[cfg(test)]
mod tests {
    use crate::game::GameBuilder;

    use super::*;

    #[test]
    fn test_remove() {
        let g = GameBuilder::default().build_with_game_state(LabGameState);
        assert_eq!(g.rewards.potions.len(), 2);
    }
}
