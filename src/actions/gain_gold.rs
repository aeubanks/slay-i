use crate::{action::Action, game::Game, relic::RelicClass};

pub struct GainGoldAction(pub i32);

impl Action for GainGoldAction {
    fn run(&self, game: &mut Game) {
        if !game.has_relic(RelicClass::Ectoplasm) {
            game.gold += self.0;
        }
    }
}

impl std::fmt::Debug for GainGoldAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gain gold {}", self.0)
    }
}
