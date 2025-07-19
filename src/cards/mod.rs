mod attacks;
mod powers;
mod skills;

use std::{cell::RefCell, rc::Rc};

use crate::card::{Card, CardBehavior, CardRarity, CardRef, CardType};

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
    // Uncommon attacks
    SearingBlow,
    // Uncommon skills
    GhostlyArmor,
    Bloodletting,
    // Uncommon powers
    Inflame,
    // Rare skills
    Impervious,
    // Other
    DebugKill,
    TestAttack,
    TestSkill,
    TestPower,
}

impl CardClass {
    pub fn is_ethereal(&self) -> bool {
        use CardClass::*;
        matches!(self, GhostlyArmor)
    }
    pub fn can_upgrade_forever(&self) -> bool {
        matches!(self, CardClass::SearingBlow)
    }
}

pub fn card(class: CardClass) -> CardRef {
    use CardClass::*;
    use CardRarity::*;
    use CardType::*;
    let (ty, rarity, cost, has_target, behavior, exhaust): (
        CardType,
        CardRarity,
        i32,
        bool,
        CardBehavior,
        bool,
    ) = match class {
        Strike => (Attack, Basic, 1, true, attacks::strike_behavior, false),
        Defend => (Skill, Basic, 1, false, skills::defend_behavior, false),
        Bash => (Attack, Basic, 2, true, attacks::bash_behavior, false),
        PommelStrike => (
            Attack,
            Common,
            1,
            true,
            attacks::pommel_strike_behavior,
            false,
        ),
        TwinStrike => (
            Attack,
            Common,
            1,
            true,
            attacks::twin_strike_behavior,
            false,
        ),
        Clothesline => (
            Attack,
            Common,
            2,
            true,
            attacks::clothesline_behavior,
            false,
        ),
        Cleave => (Attack, Common, 1, false, attacks::cleave_behavior, false),
        Thunderclap => (
            Attack,
            Common,
            1,
            false,
            attacks::thunderclap_behavior,
            false,
        ),
        SearingBlow => (
            Attack,
            Uncommon,
            2,
            true,
            attacks::searing_blow_behavior,
            false,
        ),
        GhostlyArmor => (
            Skill,
            Uncommon,
            1,
            false,
            skills::ghostly_armor_behavior,
            false,
        ),
        Bloodletting => (
            Skill,
            Uncommon,
            0,
            false,
            skills::bloodletting_behavior,
            false,
        ),
        Inflame => (Power, Uncommon, 1, false, powers::inflame_behavior, false),
        Impervious => (Skill, Rare, 2, false, skills::impervious_behavior, true),
        DebugKill => (
            Attack,
            Special,
            1,
            true,
            attacks::debug_kill_behavior,
            false,
        ),
        TestAttack => (Attack, Special, 1, false, |_, _, _| (), false),
        TestSkill => (Skill, Special, 1, false, |_, _, _| (), false),
        TestPower => (Power, Special, 1, false, |_, _, _| (), false),
    };

    Rc::new(RefCell::new(Card {
        class,
        ty,
        rarity,
        upgrade_count: 0,
        upgrade_fn: None,
        has_target,
        cost,
        behavior,
        exhaust,
    }))
}

#[cfg(test)]
pub fn upgraded_card(class: CardClass) -> CardRef {
    let c = card(class);
    c.borrow_mut().upgrade();
    c
}
