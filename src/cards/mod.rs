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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum CardClass {
    // Basic
    Strike,
    Defend,
    Bash,
    // Common attacks
    PommelStrike,
    TwinStrike,
    Clothesline,
    Cleave,
    Thunderclap,
    // Common skills
    Armaments,
    // Uncommon attacks
    SearingBlow,
    Whirlwind,
    // Uncommon skills
    GhostlyArmor,
    Bloodletting,
    // Uncommon powers
    Inflame,
    // Rare skills
    LimitBreak,
    Impervious,
    // Statuses
    Wound,
    Dazed,
    Slimed,
    Burn,
    BurnPlus,
    // Curses
    AscendersBane,
    CurseOfTheBell,
    Clumsy,
    Injury,
    Shame,
    Doubt,
    Decay,
    Regret,
    // Other
    DebugKill,
    TestAttack,
    TestSkill,
    TestPower,
}

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
    #[allow(dead_code)]
    pub fn rarity(&self) -> CardRarity {
        use CardClass::*;
        use CardRarity::*;
        match self {
            Strike | Defend | Bash => Basic,
            PommelStrike | TwinStrike | Clothesline | Cleave | Thunderclap | Armaments => Common,
            SearingBlow | Whirlwind | GhostlyArmor | Bloodletting | Inflame => Uncommon,
            Impervious | LimitBreak => Rare,
            DebugKill | TestAttack | TestSkill | TestPower | Dazed | Wound | Slimed | Burn
            | BurnPlus | AscendersBane | CurseOfTheBell | Clumsy | Injury | Shame | Doubt
            | Decay | Regret => Special,
        }
    }
    pub fn ty(&self) -> CardType {
        use CardClass::*;
        use CardType::*;
        match self {
            Strike | Bash | PommelStrike | TwinStrike | Clothesline | Cleave | Thunderclap
            | SearingBlow | Whirlwind | DebugKill | TestAttack => Attack,
            Defend | Armaments | GhostlyArmor | Bloodletting | Impervious | LimitBreak
            | TestSkill => Skill,
            Inflame | TestPower => Power,
            Dazed | Wound | Slimed | Burn | BurnPlus => Status,
            AscendersBane | CurseOfTheBell | Clumsy | Injury | Shame | Doubt | Decay | Regret => {
                Curse
            }
        }
    }
    pub fn has_target(&self) -> bool {
        use CardClass::*;
        matches!(
            self,
            Strike | Bash | PommelStrike | TwinStrike | Clothesline | SearingBlow
        )
    }
    pub fn behavior(&self) -> CardBehavior {
        use CardClass::*;
        match self {
            Strike => attacks::strike_behavior,
            Defend => skills::defend_behavior,
            Bash => attacks::bash_behavior,
            PommelStrike => attacks::pommel_strike_behavior,
            TwinStrike => attacks::twin_strike_behavior,
            Clothesline => attacks::clothesline_behavior,
            Cleave => attacks::cleave_behavior,
            Thunderclap => attacks::thunderclap_behavior,
            Armaments => skills::armaments_behavior,
            SearingBlow => attacks::searing_blow_behavior,
            Whirlwind => attacks::whirlwind_behavior,
            GhostlyArmor => skills::ghostly_armor_behavior,
            Bloodletting => skills::bloodletting_behavior,
            Inflame => powers::inflame_behavior,
            Impervious => skills::impervious_behavior,
            LimitBreak => skills::limit_break_behavior,
            DebugKill => attacks::debug_kill_behavior,
            TestAttack | TestSkill | TestPower | Dazed | Wound | Slimed | Burn | BurnPlus
            | AscendersBane | CurseOfTheBell | Clumsy | Injury | Shame | Doubt | Decay | Regret => {
                |_, _, _| ()
            }
        }
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
    pub fn base_cost(&self) -> CardCost {
        use CardClass::*;
        use CardCost::*;
        match self {
            Bloodletting | DebugKill | TestAttack | TestSkill | TestPower => Cost(0),
            Strike | Defend | PommelStrike | TwinStrike | Cleave | Thunderclap | Armaments
            | GhostlyArmor | Inflame | LimitBreak | Slimed => Cost(1),
            Bash | Clothesline | SearingBlow | Impervious => Cost(2),
            Whirlwind => X,
            Dazed | Wound | Burn | BurnPlus | AscendersBane | CurseOfTheBell | Clumsy | Injury
            | Shame | Doubt | Decay | Regret => None,
        }
    }
    pub fn base_exhaust(&self) -> bool {
        use CardClass::*;
        matches!(
            self,
            Impervious
                | LimitBreak
                | Slimed
                | Dazed
                | Wound
                | Burn
                | BurnPlus
                | AscendersBane
                | CurseOfTheBell
                | Clumsy
                | Injury
                | Shame
                | Doubt
                | Decay
                | Regret
        )
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
        exhaust: class.base_exhaust(),
    }))
}

pub fn new_card_upgraded(class: CardClass) -> CardRef {
    let c = new_card(class);
    c.borrow_mut().upgrade();
    c
}
