mod attacks;
mod curses;
mod powers;
mod skills;
mod statuses;

use std::{cell::RefCell, rc::Rc};

use crate::{
    card::{Card, CardPlayInfo, CardRef},
    game::{CreatureRef, Game},
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

macro_rules! c {
    ($($name:ident => ($rarity:expr, $ty:expr, $cost:expr, $behavior:expr, $exhausts:expr)),+,) => {
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
            #[allow(dead_code)]
            pub fn all() -> Vec<Self> {
                vec![$(Self::$name,)+]
            }
        }
    };
}

fn noop_behavior(_: &mut Game, _: Option<CreatureRef>, _: CardPlayInfo) {}

c!(
    // Basic
    Strike => (Basic, Attack, Cost(1), attacks::strike_behavior, false),
    Defend => (Basic, Skill, Cost(1), skills::defend_behavior, false),
    Bash => (Basic, Attack, Cost(2), attacks::bash_behavior, false),
    // Common attacks
    PommelStrike => (Common, Attack, Cost(1), attacks::pommel_strike_behavior, false),
    TwinStrike => (Common, Attack, Cost(1), attacks::twin_strike_behavior, false),
    Clothesline => (Common, Attack, Cost(2), attacks::clothesline_behavior, false),
    Cleave => (Common, Attack, Cost(1), attacks::cleave_behavior, false),
    Thunderclap => (Common, Attack, Cost(1), attacks::thunderclap_behavior, false),
    // Common skills
    Armaments => (Common, Skill, Cost(1), skills::armaments_behavior, false),
    // Uncommon attacks
    SearingBlow => (Uncommon, Attack, Cost(2), attacks::searing_blow_behavior, false),
    Whirlwind => (Uncommon, Attack, X, attacks::whirlwind_behavior, false),
    // Uncommon skills
    GhostlyArmor => (Uncommon, Skill, Cost(1), skills::ghostly_armor_behavior, false),
    Bloodletting => (Uncommon, Skill, Cost(0), skills::bloodletting_behavior, false),
    // Uncommon powers
    Inflame => (Uncommon, Power, Cost(1), powers::inflame_behavior, false),
    // Rare skills
    LimitBreak => (Rare, Skill, Cost(1), skills::limit_break_behavior, true),
    Impervious => (Rare, Skill, Cost(2), skills::impervious_behavior, true),
    // Colorless uncommon attacks
    SwiftStrike => (Uncommon, Attack, Cost(0), attacks::swift_strike_behavior, false),
    FlashOfSteel => (Uncommon, Attack, Cost(0), attacks::flash_of_steel_behavior, false),
    // Colorless uncommon skills
    GoodInstincts => (Uncommon, Skill, Cost(0), skills::good_instincts_behavior, false),
    Finesse => (Uncommon, Skill, Cost(0), skills::finesse_behavior, false),
    // Statuses
    Wound => (Special, Status, None, noop_behavior, true),
    Dazed => (Special, Status, None, noop_behavior, true),
    Slimed => (Special, Status, Cost(1), noop_behavior, true),
    Burn => (Special, Status, None, noop_behavior, true),
    BurnPlus => (Special, Status, None, noop_behavior, true),
    // Curses
    AscendersBane => (Special, Curse, None, noop_behavior, true),
    CurseOfTheBell => (Special, Curse, None, noop_behavior, true),
    Clumsy => (Special, Curse, None, noop_behavior, true),
    Injury => (Special, Curse, None, noop_behavior, true),
    Shame => (Special, Curse, None, noop_behavior, true),
    Doubt => (Special, Curse, None, noop_behavior, true),
    Decay => (Special, Curse, None, noop_behavior, true),
    Regret => (Special, Curse, None, noop_behavior, true),
    // Other
    DebugKill => (Special, Attack, Cost(0), attacks::debug_kill_behavior, false),
    TestAttack => (Special, Attack, Cost(0), noop_behavior, false),
    TestSkill => (Special, Skill, Cost(0), noop_behavior, false),
    TestPower => (Special, Power, Cost(0), noop_behavior, false),
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
