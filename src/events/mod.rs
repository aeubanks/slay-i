use crate::{
    cards::CardType,
    events::{
        accursed_blacksmith::AccursedBlackSmithGameState, big_fish::BigFishGameState,
        bonfire::BonfireGameState, divine_fountain::DivineFountainGameState,
        duplicator::DuplicatorGameState, face_trader::FaceTraderGameState, lab::LabGameState,
        purifier::PurifierGameState, transmorgrifier::TransmorgrifierGameState,
        upgrade::UpgradeShrineGameState, we_meet_again::WeMeetAgainGameState,
    },
    game::Game,
    relic::RelicClass,
    state::GameState,
};

pub mod accursed_blacksmith;
pub mod big_fish;
pub mod bonfire;
pub mod divine_fountain;
pub mod duplicator;
pub mod face_trader;
pub mod lab;
pub mod purifier;
pub mod transmorgrifier;
pub mod upgrade;
pub mod we_meet_again;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Event {
    AccursedBlackSmith,
    BigFish,
    Bonfire,
    DivineFountain,
    Purifier,
    Transmorgrifier,
    Upgrade,
    Duplicator,
    Designer, // TODO
    FaceTrader,
    KnowingSkull, // TODO
    Nloth,        // TODO
    Joust,        // TODO
    WomanInBlue,  // TODO
    Lab,
    WeMeetAgain,
    Falling,           // TODO
    MindBloom,         // TODO
    MoaiHead,          // TODO
    MysteriousSphere,  // TODO
    SensoryStone,      // TODO
    TombOfLordRedMask, // TODO
    WindingHalls,      // TODO
    MatchAndKeep,      // TODO
    WheelOfChange,     // TODO
    GoldenShrine,      // TODO
    Cleric,            // TODO
    DeadAdventurer,    // TODO
    GoldenIdol,        // TODO
    GoldenWing,        // TODO
    WorldOfGoop,       // TODO
    Sssserpent,        // TODO
    LivingWall,        // TODO
    Mushrooms,         // TODO
    ScrapOoze,         // TODO
    ShiningLight,      // TODO
    Addict,            // TODO
    BackToBasics,      // TODO
    Beggar,            // TODO
    Colosseum,         // TODO
    CursedTome,        // TODO
    DrugDealer,        // TODO
    ForgottenAltar,    // TODO
    Ghosts,            // TODO
    MaskedBandits,     // TODO
    Nest,              // TODO
    Library,           // TODO
    Mausoleum,         // TODO
    Vampires,          // TODO
}

impl Event {
    pub fn game_state(&self, game: &mut Game) -> Box<dyn GameState> {
        use Event::*;
        match self {
            AccursedBlackSmith => Box::new(AccursedBlackSmithGameState),
            BigFish => Box::new(BigFishGameState::new(game)),
            Bonfire => Box::new(BonfireGameState),
            DivineFountain => Box::new(DivineFountainGameState),
            Purifier => Box::new(PurifierGameState),
            Transmorgrifier => Box::new(TransmorgrifierGameState),
            Upgrade => Box::new(UpgradeShrineGameState),
            FaceTrader => Box::new(FaceTraderGameState::new(game)),
            Lab => Box::new(LabGameState),
            WeMeetAgain => Box::new(WeMeetAgainGameState::new(game)),
            Duplicator => Box::new(DuplicatorGameState),
            _ => todo!(),
        }
    }
    pub fn can_spawn(&self, game: &Game) -> bool {
        use Event::*;
        match self {
            DivineFountain => game.master_deck.iter().any(|c| {
                let c = c.borrow();
                c.can_remove_from_master_deck() && c.class.ty() == CardType::Curse
            }),
            FaceTrader => game.is_in_act(1),
            Duplicator => !game.is_in_act(1),
            KnowingSkull => game.is_in_act(2) && game.player.cur_hp > 12,
            Designer => !game.is_in_act(1) && game.gold >= 75,
            Nloth => game.is_in_act(3) && game.relics.len() >= 2,
            Joust => game.is_in_act(3) && game.gold >= 50,
            DeadAdventurer | Mushrooms => game.floor > 6,
            Cleric => game.gold >= 35,
            Beggar => game.gold >= 75,
            Colosseum => game.floor > 26,
            MoaiHead => {
                game.has_relic(RelicClass::GoldenIdol)
                    || game.player.cur_hp as f32 / game.player.max_hp as f32 <= 0.5
            }
            _ => true,
        }
    }
}
