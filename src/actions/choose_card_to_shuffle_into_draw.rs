use crate::{action::Action, cards::random_red_in_combat, game::Game, state::GameState};

pub struct ChooseCardToShuffleIntoDrawAction();

impl Action for ChooseCardToShuffleIntoDrawAction {
    fn run(&self, game: &mut Game) {
        let mut classes = Vec::new();
        while classes.len() < 3 {
            let c = random_red_in_combat(&mut game.rng);
            if !classes.contains(&c) {
                classes.push(c);
            }
        }
        game.state.push_state(GameState::Nilrys { classes });
    }
}

impl std::fmt::Debug for ChooseCardToShuffleIntoDrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose card to shuffle into draw")
    }
}
