use crate::{
    events::{
        accursed_blacksmith::AccursedBlackSmithGameState, big_fish::BigFishGameState,
        bonfire::BonfireGameState, divine_fountain::DivineFountainGameState,
        purifier::PurifierGameState, transmorgrifier::TransmorgrifierGameState,
    },
    game::Game,
    state::GameState,
};

pub mod accursed_blacksmith;
pub mod big_fish;
pub mod bonfire;
pub mod divine_fountain;
pub mod purifier;
pub mod transmorgrifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Event {
    AccursedBlackSmith,
    BigFish,
    Bonfire,
    DivineFountain,
    Purifier,
    Transmorgrifier,
}

impl Event {
    pub fn game_state(&self, game: &Game) -> Box<dyn GameState> {
        use Event::*;
        match self {
            AccursedBlackSmith => Box::new(AccursedBlackSmithGameState),
            BigFish => Box::new(BigFishGameState::new(game)),
            Bonfire => Box::new(BonfireGameState),
            DivineFountain => Box::new(DivineFountainGameState),
            Purifier => Box::new(PurifierGameState),
            Transmorgrifier => Box::new(TransmorgrifierGameState),
        }
    }
}
