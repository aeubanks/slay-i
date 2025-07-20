use std::{cell::RefCell, rc::Rc};

use crate::cards::CardClass;

#[derive(Clone, Copy)]
pub struct CardPlayInfo {
    pub upgraded: bool,
    pub upgrade_count: i32,
}

#[derive(Clone)]
pub struct Card {
    pub class: CardClass,
    pub upgrade_count: i32,
    pub cost: i32,
    pub exhaust: bool,
}

impl Card {
    pub fn can_upgrade(&self) -> bool {
        self.upgrade_count == 0 || self.class.can_upgrade_forever()
    }
    pub fn upgrade(&mut self) {
        assert!(self.can_upgrade());
        self.upgrade_count += 1;
        if let Some(f) = self.class.upgrade_fn() {
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

#[cfg(test)]
pub fn clone_card(c: &CardRef) -> CardRef {
    Rc::new(RefCell::new(c.borrow().clone()))
}
