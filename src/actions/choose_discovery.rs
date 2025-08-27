use crate::{
    action::Action,
    cards::random_red_in_combat,
    game::{Game, GameState},
};

pub struct ChooseDiscoveryAction();

impl Action for ChooseDiscoveryAction {
    fn run(&self, game: &mut Game) {
        let mut classes = Vec::new();
        while classes.len() < 3 {
            let c = random_red_in_combat(&mut game.rng);
            if !classes.contains(&c) {
                classes.push(c);
            }
        }
        game.state = GameState::Discovery(classes);
    }
}

impl std::fmt::Debug for ChooseDiscoveryAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose discovery")
    }
}
