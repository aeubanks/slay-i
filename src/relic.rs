use lazy_static::lazy_static;

use crate::{
    actions::{
        block::BlockAction, damage::DamageAction, draw::DrawAction, heal::HealAction,
        play_card::PlayCardAction,
    },
    cards::CardType,
    game::{CreatureRef, Rand},
    queue::ActionQueue,
    rng::rand_slice,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum RelicRarity {
    Starter,
    Common,
    Uncommon,
    Rare,
    Shop,
    Event,
    Boss,
}

macro_rules! r {
    ($($name:ident => $rarity:expr),+,) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum RelicClass {
            $(
                $name,
            )+
        }
        impl RelicClass {
            pub fn rarity(&self) -> RelicRarity {
                use RelicRarity::*;
                match self {
                    $(Self::$name => $rarity,)+
                }
            }
        }
        impl RelicClass {
            pub fn all() -> Vec<Self> {
                vec![$(Self::$name,)+]
            }
        }
    };
}

r!(
    BurningBlood => Starter,

    Anchor => Common,
    BagOfPrep => Common,
    BloodVial => Common,

    BlueCandle => Uncommon,
    HornCleat => Uncommon,

    CaptainsWheel => Rare,

    SacredBark => Boss,

    MedicalKit => Shop,
);

type RelicCallback = fn(&mut i32, &mut ActionQueue);
type RelicCardCallback = fn(&mut i32, &mut ActionQueue, &PlayCardAction);

impl RelicClass {
    pub fn pre_combat(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BloodVial => Some(blood_vial),
            HornCleat | CaptainsWheel => Some(set_value_zero),
            _ => None,
        }
    }
    pub fn combat_finish(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BurningBlood => Some(burning_blood),
            _ => None,
        }
    }
    pub fn combat_start_pre_draw(&self) -> Option<RelicCallback> {
        None
    }
    pub fn combat_start_post_draw(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BagOfPrep => Some(bag_of_prep),
            Anchor => Some(anchor),
            _ => None,
        }
    }
    pub fn on_card_played(&self) -> Option<RelicCardCallback> {
        use RelicClass::*;
        match self {
            BlueCandle => Some(blue_candle),
            _ => None,
        }
    }
    pub fn turn_start(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            HornCleat => Some(horn_cleat),
            CaptainsWheel => Some(captains_wheel),
            _ => None,
        }
    }
    pub fn turn_end(&self) -> Option<RelicCallback> {
        None
    }
}

fn set_value_zero(v: &mut i32, _: &mut ActionQueue) {
    *v = 0;
}

fn burning_blood(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: 6,
    });
}

fn blood_vial(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: 2,
    });
}

fn bag_of_prep(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(DrawAction(2));
}

fn anchor(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(BlockAction::player_flat_amount(10));
}

fn horn_cleat(v: &mut i32, queue: &mut ActionQueue) {
    *v += 1;
    if *v == 2 {
        queue.push_bot(BlockAction::player_flat_amount(14));
    }
}

fn captains_wheel(v: &mut i32, queue: &mut ActionQueue) {
    *v += 1;
    if *v == 3 {
        queue.push_bot(BlockAction::player_flat_amount(18));
    }
}

fn blue_candle(_: &mut i32, queue: &mut ActionQueue, play: &PlayCardAction) {
    if play.card.borrow().class.ty() == CardType::Curse {
        queue.push_bot(DamageAction::lose_hp(1, CreatureRef::player()));
    }
}

pub struct Relic {
    class: RelicClass,
    value: i32,
}

impl Relic {
    pub fn get_class(&self) -> RelicClass {
        self.class
    }
}

macro_rules! trigger {
    ($name:ident) => {
        pub fn $name(&mut self, queue: &mut ActionQueue) {
            if let Some(f) = self.class.$name() {
                f(&mut self.value, queue)
            }
        }
    };
}
macro_rules! trigger_card {
    ($name:ident) => {
        pub fn $name(&mut self, queue: &mut ActionQueue, play: &PlayCardAction) {
            if let Some(f) = self.class.$name() {
                f(&mut self.value, queue, play)
            }
        }
    };
}

impl Relic {
    trigger!(pre_combat);
    trigger!(combat_start_pre_draw);
    trigger!(combat_start_post_draw);
    trigger!(turn_start);
    trigger!(turn_end);
    trigger!(combat_finish);
    trigger_card!(on_card_played);
}

pub fn new_relic(class: RelicClass) -> Relic {
    Relic { class, value: 0 }
}

lazy_static! {
    static ref ALL_COMMON: Vec<RelicClass> = RelicClass::all()
        .into_iter()
        .filter(|r| r.rarity() == RelicRarity::Common)
        .collect();
}

pub fn random_common_relic(rng: &mut Rand) -> RelicClass {
    rand_slice(rng, &ALL_COMMON)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::block::BlockAction,
        cards::CardClass,
        game::{GameBuilder, Move},
        status::Status,
    };

    #[test]
    fn test_burning_blood() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::DebugKill)
            .add_relic(RelicClass::BurningBlood)
            .build_combat();
        let hp = g.player.creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.player.creature.cur_hp, hp + 6);
    }

    #[test]
    fn test_blood_vial() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::BloodVial)
            .set_player_hp(50)
            .build_combat();
        assert_eq!(g.player.creature.cur_hp, 52);
    }

    #[test]
    fn test_bag_of_prep() {
        let g = GameBuilder::default()
            .ironclad_starting_deck()
            .add_relic(RelicClass::BagOfPrep)
            .build_combat();
        assert_eq!(g.hand.len(), 7);
    }

    #[test]
    fn test_medical_kit() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Wound)
            .add_card(CardClass::Dazed)
            .add_relic(RelicClass::MedicalKit)
            .set_player_hp(50)
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
        assert_eq!(g.player.creature.cur_hp, 50);
        assert_eq!(g.exhaust_pile.len(), 2);
        assert_eq!(g.discard_pile.len(), 0);
    }

    #[test]
    fn test_blue_candle() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::AscendersBane)
            .add_card(CardClass::Injury)
            .add_relic(RelicClass::BlueCandle)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction::player_flat_amount(5));
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
        assert_eq!(g.player.creature.cur_hp, 48);
        assert_eq!(g.exhaust_pile.len(), 2);
        assert_eq!(g.discard_pile.len(), 0);
    }

    #[test]
    fn test_anchor() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Anchor)
            .build_combat();
        assert_eq!(g.player.creature.block, 10);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
    }

    #[test]
    fn test_anchor_dexterity() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Anchor)
            .add_player_status(Status::Dexterity, 55)
            .build_combat();
        assert_eq!(g.player.creature.block, 10);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
    }

    #[test]
    fn test_horn_cleat() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::HornCleat)
            .build_combat();
        assert_eq!(g.player.creature.block, 0);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 14);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
    }

    #[test]
    fn test_captains_wheel() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::CaptainsWheel)
            .build_combat();
        assert_eq!(g.player.creature.block, 0);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 18);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
    }
}
