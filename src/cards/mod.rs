mod attacks;
mod skills;

use std::{cell::RefCell, rc::Rc};

use crate::card::{Card, CardBehavior, CardRarity, CardRef, CardType};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum CardClass {
    Strike,
    Defend,
    Bash,
    PommelStrike,
    Clothesline,
    SearingBlow,
    Impervious,
    DebugKill,
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
        Clothesline => (
            Attack,
            Common,
            2,
            true,
            attacks::clothesline_behavior,
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
        Impervious => (Skill, Rare, 2, false, skills::impervious_behavior, true),
        DebugKill => (
            Attack,
            Special,
            1,
            true,
            attacks::debug_kill_behavior,
            false,
        ),
    };

    Rc::new(RefCell::new(Card {
        class,
        ty,
        rarity,
        upgrade_count: 0,
        upgrade_fn: None,
        can_upgrade_forever: class == SearingBlow,
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
