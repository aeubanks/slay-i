mod attacks;
mod curses;
mod powers;
mod skills;
mod statuses;

use lazy_static::lazy_static;
use rand::Rng;
use std::{cell::RefCell, rc::Rc};

use crate::{
    card::{Card, CardPlayInfo, CardRef},
    game::{CreatureRef, Game, Rand},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum CardRarity {
    Basic,
    Common,
    Uncommon,
    Rare,
    Special,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum CardColor {
    Red,
    Colorless,
    Curse,
    Special,
}

macro_rules! c {
    ($($name:ident => ($rarity:expr, $ty:expr, $color:expr, $cost:expr, $behavior:expr, $exhausts:expr)),+,) => {
        #[allow(dead_code)]
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum CardClass {
            $(
                $name,
            )+
        }
        impl CardClass {
            #[allow(dead_code)]
            pub fn rarity(&self) -> CardRarity {
                use CardRarity::*;
                match self {
                    $(Self::$name => $rarity,)+
                }
            }
            pub fn ty(&self) -> CardType {
                use CardType::*;
                match self {
                    $(Self::$name => $ty,)+
                }
            }
            pub fn color(&self) -> CardColor {
                use CardColor::*;
                match self {
                    $(Self::$name => $color,)+
                }
            }
            pub fn base_cost(&self) -> CardCost {
                use CardCost::*;
                match self {
                    $(Self::$name => $cost,)+
                }
            }
            pub fn behavior(&self) -> CardBehavior {
                match self {
                    $(Self::$name => $behavior,)+
                }
            }
            pub fn base_exhausts(&self) -> bool {
                match self {
                    $(Self::$name => $exhausts,)+
                }
            }
        }
        impl CardClass {
            pub fn all() -> Vec<Self> {
                vec![$(Self::$name,)+]
            }
        }
    };
}

fn noop_behavior(_: &mut Game, _: Option<CreatureRef>, _: CardPlayInfo) {}

c!(
    // Basic
    Strike => (Basic, Attack, Red, Cost(1), attacks::strike_behavior, false),
    Defend => (Basic, Skill, Red, Cost(1), skills::defend_behavior, false),
    Bash => (Basic, Attack, Red, Cost(2), attacks::bash_behavior, false),
    // Common attacks
    PommelStrike => (Common, Attack, Red, Cost(1), attacks::pommel_strike_behavior, false),
    TwinStrike => (Common, Attack, Red, Cost(1), attacks::twin_strike_behavior, false),
    Clothesline => (Common, Attack, Red, Cost(2), attacks::clothesline_behavior, false),
    Cleave => (Common, Attack, Red, Cost(1), attacks::cleave_behavior, false),
    Thunderclap => (Common, Attack, Red, Cost(1), attacks::thunderclap_behavior, false),
    // Common skills
    Armaments => (Common, Skill, Red, Cost(1), skills::armaments_behavior, false),
    // Uncommon attacks
    SearingBlow => (Uncommon, Attack, Red, Cost(2), attacks::searing_blow_behavior, false),
    Whirlwind => (Uncommon, Attack, Red, X, attacks::whirlwind_behavior, false),
    // Uncommon skills
    GhostlyArmor => (Uncommon, Skill, Red, Cost(1), skills::ghostly_armor_behavior, false),
    Bloodletting => (Uncommon, Skill, Red, Cost(0), skills::bloodletting_behavior, false),
    // Uncommon powers
    Inflame => (Uncommon, Power, Red, Cost(1), powers::inflame_behavior, false),
    // Rare skills
    LimitBreak => (Rare, Skill, Red, Cost(1), skills::limit_break_behavior, true),
    Impervious => (Rare, Skill, Red, Cost(2), skills::impervious_behavior, true),
    // Colorless uncommon attacks
    SwiftStrike => (Uncommon, Attack, Colorless, Cost(0), attacks::swift_strike_behavior, false),
    FlashOfSteel => (Uncommon, Attack, Colorless, Cost(0), attacks::flash_of_steel_behavior, false),
    // Colorless uncommon skills
    GoodInstincts => (Uncommon, Skill, Colorless, Cost(0), skills::good_instincts_behavior, false),
    Finesse => (Uncommon, Skill, Colorless, Cost(0), skills::finesse_behavior, false),
    // Statuses
    Wound => (Special, Status, Special, None, noop_behavior, true),
    Dazed => (Special, Status, Special, None, noop_behavior, true),
    Slimed => (Special, Status, Special, Cost(1), noop_behavior, true),
    Burn => (Special, Status, Special, None, noop_behavior, true),
    BurnPlus => (Special, Status, Special, None, noop_behavior, true),
    // Curses
    AscendersBane => (Special, Curse, Curse, None, noop_behavior, true),
    CurseOfTheBell => (Special, Curse, Curse, None, noop_behavior, true),
    Clumsy => (Special, Curse, Curse, None, noop_behavior, true),
    Injury => (Special, Curse, Curse, None, noop_behavior, true),
    Shame => (Special, Curse, Curse, None, noop_behavior, true),
    Doubt => (Special, Curse, Curse, None, noop_behavior, true),
    Decay => (Special, Curse, Curse, None, noop_behavior, true),
    Regret => (Special, Curse, Curse, None, noop_behavior, true),
    // Other
    DebugKill => (Special, Attack, Special, Cost(0), attacks::debug_kill_behavior, false),
    TestAttack => (Special, Attack, Special, Cost(0), noop_behavior, false),
    TestSkill => (Special, Skill, Special, Cost(0), noop_behavior, false),
    TestPower => (Special, Power, Special, Cost(0), noop_behavior, false),
);

pub type CardBehavior = fn(&mut Game, Option<CreatureRef>, CardPlayInfo);
pub type CardEndOfTurnBehavior = fn(&mut Game);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CardCost {
    Cost(i32),
    X,
    None,
}

impl CardClass {
    pub fn can_upgrade_forever(&self) -> bool {
        matches!(self, CardClass::SearingBlow)
    }
    pub fn can_remove_from_master_deck(&self) -> bool {
        !matches!(self, CardClass::AscendersBane | CardClass::CurseOfTheBell)
    }
    pub fn has_target(&self) -> bool {
        use CardClass::*;
        matches!(
            self,
            Strike
                | Bash
                | PommelStrike
                | TwinStrike
                | Clothesline
                | SearingBlow
                | SwiftStrike
                | FlashOfSteel
                | DebugKill
        )
    }
    pub fn end_of_turn_in_hand_behavior(&self) -> Option<CardEndOfTurnBehavior> {
        use CardClass::*;
        match self {
            Burn => Some(statuses::burn_behavior),
            BurnPlus => Some(statuses::burn_plus_behavior),
            Regret => Some(curses::regret_behavior),
            Decay => Some(curses::decay_behavior),
            Shame => Some(curses::shame_behavior),
            Doubt => Some(curses::doubt_behavior),
            _ => None,
        }
    }
    pub fn is_ethereal(&self) -> bool {
        use CardClass::*;
        matches!(self, GhostlyArmor | Dazed | AscendersBane | Clumsy)
    }
    // Change (cost, exhaust)
    pub fn upgrade_fn(&self) -> Option<fn(&mut CardCost, &mut bool)> {
        use CardClass::*;
        match self {
            LimitBreak => Some(|_, exhaust| *exhaust = false),
            _ => None,
        }
    }
}

pub fn new_card(class: CardClass) -> CardRef {
    Rc::new(RefCell::new(Card {
        class,
        upgrade_count: 0,
        cost: class.base_cost(),
        exhaust: class.base_exhausts(),
    }))
}

pub fn new_card_upgraded(class: CardClass) -> CardRef {
    let c = new_card(class);
    c.borrow_mut().upgrade();
    c
}

lazy_static! {
    static ref ALL_COLORLESS: Vec<CardClass> = CardClass::all()
        .into_iter()
        .filter(|c| c.color() == CardColor::Colorless)
        .collect();
    static ref ALL_UNCOMMON_COLORLESS: Vec<CardClass> = ALL_COLORLESS
        .iter()
        .copied()
        .filter(|c| c.rarity() == CardRarity::Uncommon)
        .collect();
    static ref ALL_NON_BASIC_RED: Vec<CardClass> = CardClass::all()
        .iter()
        .copied()
        .filter(|c| c.color() == CardColor::Red)
        .filter(|c| c.rarity() != CardRarity::Basic)
        .collect();
    static ref ALL_CURSES: Vec<CardClass> = CardClass::all()
        .iter()
        .copied()
        .filter(|c| c.ty() == CardType::Curse)
        .filter(|&c| c != CardClass::AscendersBane && c != CardClass::CurseOfTheBell)
        .collect();
}

fn random_red(rng: &mut Rand) -> CardClass {
    let i = rng.random_range(0..ALL_NON_BASIC_RED.len());
    ALL_NON_BASIC_RED[i]
}

fn random_colorless(rng: &mut Rand) -> CardClass {
    let i = rng.random_range(0..ALL_COLORLESS.len());
    ALL_COLORLESS[i]
}

pub fn random_uncommon_colorless(rng: &mut Rand) -> CardClass {
    let i = rng.random_range(0..ALL_UNCOMMON_COLORLESS.len());
    ALL_UNCOMMON_COLORLESS[i]
}

fn random_curse(rng: &mut Rand) -> CardClass {
    let i = rng.random_range(0..ALL_CURSES.len());
    ALL_CURSES[i]
}

pub fn transformed(class: CardClass, rng: &mut Rand) -> CardClass {
    loop {
        let new = match class.color() {
            CardColor::Red => random_red(rng),
            CardColor::Colorless => random_colorless(rng),
            CardColor::Curse => random_curse(rng),
            CardColor::Special => unreachable!(),
        };
        if new != class {
            return new;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cards::{CardClass, CardColor, CardRarity, transformed},
        game::Rand,
    };

    #[test]
    fn test_transformed() {
        let mut rng = Rand::default();
        for _ in 0..100 {
            {
                let t = transformed(CardClass::Strike, &mut rng);
                assert_eq!(t.color(), CardColor::Red);
                assert!(
                    t.rarity() == CardRarity::Common
                        || t.rarity() == CardRarity::Uncommon
                        || t.rarity() == CardRarity::Rare
                );
            }
            {
                let t = transformed(CardClass::Impervious, &mut rng);
                assert_eq!(t.color(), CardColor::Red);
                assert!(
                    t.rarity() == CardRarity::Common
                        || t.rarity() == CardRarity::Uncommon
                        || t.rarity() == CardRarity::Rare
                );
                assert_ne!(t, CardClass::Impervious);
            }
            {
                let t = transformed(CardClass::FlashOfSteel, &mut rng);
                assert_eq!(t.color(), CardColor::Colorless);
                assert!(t.rarity() == CardRarity::Uncommon || t.rarity() == CardRarity::Rare);
                assert_ne!(t, CardClass::FlashOfSteel);
            }
            {
                let t = transformed(CardClass::AscendersBane, &mut rng);
                assert_eq!(t.color(), CardColor::Curse);
                assert_eq!(t.rarity(), CardRarity::Special);
            }
            {
                let t = transformed(CardClass::Injury, &mut rng);
                assert_eq!(t.color(), CardColor::Curse);
                assert_eq!(t.rarity(), CardRarity::Special);
                assert_ne!(t, CardClass::AscendersBane);
                assert_ne!(t, CardClass::Injury);
            }
        }
    }
}
