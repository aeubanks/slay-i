mod attacks;
mod curses;
mod powers;
mod skills;
mod statuses;

use lazy_static::lazy_static;
use std::{cell::RefCell, rc::Rc};

use crate::{
    card::{Card, CardPlayInfo, CardRef},
    game::{CreatureRef, Game, Rand},
    rng::rand_slice,
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
                fn cost(c: i32) -> CardCost {
                    CardCost::Cost{base_cost:c,temporary_cost: None}
                }
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
    Strike => (Basic, Attack, Red, cost(1), attacks::strike_behavior, false),
    Defend => (Basic, Skill, Red, cost(1), skills::defend_behavior, false),
    Bash => (Basic, Attack, Red, cost(2), attacks::bash_behavior, false),
    // Common attacks
    PommelStrike => (Common, Attack, Red, cost(1), attacks::pommel_strike_behavior, false),
    TwinStrike => (Common, Attack, Red, cost(1), attacks::twin_strike_behavior, false),
    Clothesline => (Common, Attack, Red, cost(2), attacks::clothesline_behavior, false),
    Cleave => (Common, Attack, Red, cost(1), attacks::cleave_behavior, false),
    Thunderclap => (Common, Attack, Red, cost(1), attacks::thunderclap_behavior, false),
    // Common skills
    Armaments => (Common, Skill, Red, cost(1), skills::armaments_behavior, false),
    // Uncommon attacks
    SearingBlow => (Uncommon, Attack, Red, cost(2), attacks::searing_blow_behavior, false),
    Whirlwind => (Uncommon, Attack, Red, X, attacks::whirlwind_behavior, false),
    Rampage => (Uncommon, Attack, Red, cost(1), attacks::rampage_behavior, false),
    // Uncommon skills
    GhostlyArmor => (Uncommon, Skill, Red, cost(1), skills::ghostly_armor_behavior, false),
    Bloodletting => (Uncommon, Skill, Red, cost(0), skills::bloodletting_behavior, false),
    Sentinel => (Uncommon, Skill, Red, cost(1), skills::sentinel_behavior, false),
    // Uncommon powers
    Inflame => (Uncommon, Power, Red, cost(1), powers::inflame_behavior, false),
    FeelNoPain => (Uncommon, Power, Red, cost(1), powers::feel_no_pain_behavior, false),
    DarkEmbrace => (Uncommon, Power, Red, cost(2), powers::dark_embrace_behavior, false),
    // Rare skills
    LimitBreak => (Rare, Skill, Red, cost(1), skills::limit_break_behavior, true),
    Impervious => (Rare, Skill, Red, cost(2), skills::impervious_behavior, true),
    // Rare powers
    Brutality => (Rare, Power, Red, cost(0), powers::brutality_behavior, false),
    // Colorless uncommon attacks
    SwiftStrike => (Uncommon, Attack, Colorless, cost(0), attacks::swift_strike_behavior, false),
    FlashOfSteel => (Uncommon, Attack, Colorless, cost(0), attacks::flash_of_steel_behavior, false),
    DramaticEntrance => (Uncommon, Attack, Colorless, cost(0), attacks::dramatic_entrance_behavior, true),
    MindBlast => (Uncommon, Attack, Colorless, cost(2), attacks::mind_blast_behavior, false),
    // Colorless uncommon skills
    GoodInstincts => (Uncommon, Skill, Colorless, cost(0), skills::good_instincts_behavior, false),
    Finesse => (Uncommon, Skill, Colorless, cost(0), skills::finesse_behavior, false),
    Enlightenment => (Uncommon, Skill, Colorless, cost(0), skills::enlightenment_behavior, false),
    // Statuses
    Wound => (Special, Status, Special, Zero, noop_behavior, true),
    Dazed => (Special, Status, Special, Zero, noop_behavior, true),
    Slimed => (Special, Status, Special, cost(1), noop_behavior, true),
    Burn => (Special, Status, Special, Zero, noop_behavior, true),
    BurnPlus => (Special, Status, Special, Zero, noop_behavior, true),
    // Curses
    AscendersBane => (Special, Curse, Curse, Zero, noop_behavior, true),
    CurseOfTheBell => (Special, Curse, Curse, Zero, noop_behavior, true),
    Clumsy => (Special, Curse, Curse, Zero, noop_behavior, true),
    Injury => (Special, Curse, Curse, Zero, noop_behavior, true),
    Writhe => (Special, Curse, Curse, Zero, noop_behavior, true),
    Shame => (Special, Curse, Curse, Zero, noop_behavior, true),
    Doubt => (Special, Curse, Curse, Zero, noop_behavior, true),
    Decay => (Special, Curse, Curse, Zero, noop_behavior, true),
    Regret => (Special, Curse, Curse, Zero, noop_behavior, true),
    // Other
    DebugKill => (Special, Attack, Special, cost(0), attacks::debug_kill_behavior, false),
    TestAttack => (Special, Attack, Special, cost(0), noop_behavior, false),
    TestSkill => (Special, Skill, Special, cost(0), noop_behavior, false),
    TestPower => (Special, Power, Special, cost(0), noop_behavior, false),
);

pub type CardBehavior = fn(&mut Game, Option<CreatureRef>, CardPlayInfo);
pub type CardEndOfTurnBehavior = fn(&mut Game);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CardCost {
    Cost {
        base_cost: i32,
        temporary_cost: Option<i32>,
    },
    X,
    Zero,
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
            DarkEmbrace => Some(|cost, _| match cost {
                CardCost::Cost {
                    base_cost,
                    temporary_cost: _,
                } => *base_cost = 1,
                _ => unreachable!(),
            }),
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
        times_played: 0,
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
    rand_slice(rng, &ALL_NON_BASIC_RED)
}

fn random_colorless(rng: &mut Rand) -> CardClass {
    rand_slice(rng, &ALL_COLORLESS)
}

pub fn random_uncommon_colorless(rng: &mut Rand) -> CardClass {
    rand_slice(rng, &ALL_UNCOMMON_COLORLESS)
}

fn random_curse(rng: &mut Rand) -> CardClass {
    rand_slice(rng, &ALL_CURSES)
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
        game::{GameBuilder, Rand},
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

    #[test]
    fn test_innate() {
        let g = GameBuilder::default()
            .add_card(CardClass::MindBlast)
            .add_card_upgraded(CardClass::Brutality)
            .add_card(CardClass::DramaticEntrance)
            .add_card(CardClass::Writhe)
            .add_cards(CardClass::Strike, 50)
            .build_combat();
        assert!(
            g.hand
                .iter()
                .any(|c| c.borrow().class == CardClass::MindBlast)
        );
        assert!(
            g.hand
                .iter()
                .any(|c| c.borrow().class == CardClass::Brutality)
        );
        assert!(
            g.hand
                .iter()
                .any(|c| c.borrow().class == CardClass::DramaticEntrance)
        );
        assert!(g.hand.iter().any(|c| c.borrow().class == CardClass::Writhe));
    }

    #[test]
    fn test_innate2() {
        {
            let g = GameBuilder::default()
                .add_cards(CardClass::DramaticEntrance, 7)
                .add_cards(CardClass::Strike, 50)
                .build_combat();
            assert_eq!(g.hand.len(), 7);
        }
        {
            let g = GameBuilder::default()
                .add_cards(CardClass::DramaticEntrance, 11)
                .add_cards(CardClass::Strike, 50)
                .build_combat();
            assert_eq!(g.hand.len(), 10);
        }
    }
}
