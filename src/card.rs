use std::{cell::RefCell, rc::Rc};

use crate::{
    cards::CardClass,
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

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct CardPlayInfo {
    pub upgraded: bool,
    pub upgrade_count: i32,
    pub played_count: i32,
}

pub type CardBehavior = fn(&mut Game, Option<CreatureRef>, CardPlayInfo);

#[allow(dead_code)]
#[derive(Clone)]
pub struct Card {
    pub class: CardClass,
    pub ty: CardType,
    pub rarity: CardRarity,
    pub upgrade_count: i32,
    pub upgrade_fn: Option<fn(&mut i32)>,
    pub cost: i32,
    pub has_target: bool,
    pub exhaust: bool,
    pub behavior: CardBehavior,
}

#[cfg(test)]
impl Card {
    pub fn can_upgrade(&self) -> bool {
        self.upgrade_count == 0 || self.class.can_upgrade_forever()
    }
    pub fn upgrade(&mut self) {
        assert!(self.can_upgrade());
        self.upgrade_count += 1;
        if let Some(f) = self.upgrade_fn {
            f(&mut self.cost);
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.class)?;
        for _ in 0..(self.upgrade_count) {
            write!(f, "+")?;
        }
        write!(f, ", {} cost", self.cost)?;
        Ok(())
    }
}

pub type CardRef = Rc<RefCell<Card>>;
pub type CardPile = Vec<CardRef>;

pub fn clone_card(c: &CardRef) -> CardRef {
    Rc::new(RefCell::new(c.borrow().clone()))
}
