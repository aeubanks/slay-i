mod attacks;
mod powers;
mod skills;

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
    // Uncommon skills
    GhostlyArmor,
    Bloodletting,
    // Uncommon powers
    Inflame,
    // Rare skills
    LimitBreak,
    Impervious,
    // Other
    DebugKill,
    TestAttack,
    TestSkill,
    TestPower,
}

pub type CardBehavior = fn(&mut Game, Option<CreatureRef>, CardPlayInfo);

impl CardClass {
    pub fn is_ethereal(&self) -> bool {
        use CardClass::*;
        matches!(self, GhostlyArmor)
    }
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
            SearingBlow | GhostlyArmor | Bloodletting | Inflame => Uncommon,
            Impervious | LimitBreak => Rare,
            DebugKill | TestAttack | TestSkill | TestPower => Special,
        }
    }
    pub fn ty(&self) -> CardType {
        use CardClass::*;
        use CardType::*;
        match self {
            Strike | Bash | PommelStrike | TwinStrike | Clothesline | Cleave | Thunderclap
            | SearingBlow | DebugKill | TestAttack => Attack,
            Defend | Armaments | GhostlyArmor | Bloodletting | Impervious | LimitBreak
            | TestSkill => Skill,
            Inflame | TestPower => Power,
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
            GhostlyArmor => skills::ghostly_armor_behavior,
            Bloodletting => skills::bloodletting_behavior,
            Inflame => powers::inflame_behavior,
            Impervious => skills::impervious_behavior,
            LimitBreak => skills::limit_break_behavior,
            DebugKill => attacks::debug_kill_behavior,
            TestAttack => |_, _, _| (),
            TestSkill => |_, _, _| (),
            TestPower => |_, _, _| (),
        }
    }
    pub fn base_cost(&self) -> i32 {
        use CardClass::*;
        match self {
            Bloodletting | DebugKill | TestAttack | TestSkill | TestPower => 0,
            Strike | Defend | PommelStrike | TwinStrike | Cleave | Thunderclap | Armaments
            | GhostlyArmor | Inflame | LimitBreak => 1,
            Bash | Clothesline | SearingBlow | Impervious => 2,
        }
    }
    pub fn base_exhaust(&self) -> bool {
        use CardClass::*;
        matches!(self, Impervious | LimitBreak)
    }
    // Change (cost, exhaust)
    pub fn upgrade_fn(&self) -> Option<fn(&mut i32, &mut bool)> {
        use CardClass::*;
        match self {
            LimitBreak => Some(|_, exhaust| *exhaust = false),
            _ => None,
        }
    }
}

pub fn card(class: CardClass) -> CardRef {
    Rc::new(RefCell::new(Card {
        class,
        upgrade_count: 0,
        cost: class.base_cost(),
        exhaust: class.base_exhaust(),
    }))
}

#[cfg(test)]
pub fn upgraded_card(class: CardClass) -> CardRef {
    let c = card(class);
    c.borrow_mut().upgrade();
    c
}
