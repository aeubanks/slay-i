use std::{cell::RefCell, rc::Rc};

use crate::cards::{CardClass, CardCost, CardType};

#[derive(Clone, Copy)]
pub struct CardPlayInfo {
    pub upgraded: bool,
    pub upgrade_count: i32,
    pub times_played: i32,
}

#[derive(Clone)]
pub struct Card {
    pub class: CardClass,
    pub upgrade_count: i32,
    pub cost: CardCost,
    pub exhaust: bool,
    pub times_played: i32,
}

impl Card {
    pub fn can_upgrade(&self) -> bool {
        (self.upgrade_count == 0 || self.class.can_upgrade_forever())
            && !matches!(self.class.ty(), CardType::Status | CardType::Curse)
    }
    pub fn upgrade(&mut self) {
        assert!(self.can_upgrade());
        self.upgrade_count += 1;
        if let Some(f) = self.class.upgrade_fn() {
            f(&mut self.cost, &mut self.exhaust);
        }
    }
    pub fn is_innate(&self) -> bool {
        use CardClass::*;
        match self.class {
            Brutality => self.upgrade_count != 0,
            DramaticEntrance | MindBlast | Writhe => true,
            _ => false,
        }
    }
    pub fn clear_temporary(&mut self) {
        match &mut self.cost {
            CardCost::Cost {
                base_cost: _,
                temporary_cost,
            } => {
                *temporary_cost = None;
            }
            CardCost::X | CardCost::Zero => {}
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.class)?;
        for _ in 0..(self.upgrade_count) {
            write!(f, "+")?;
        }
        match self.cost {
            CardCost::Zero | CardCost::X => {}
            CardCost::Cost {
                base_cost,
                temporary_cost,
            } => {
                write!(f, ", {base_cost} cost")?;
                if let Some(temporary_cost) = temporary_cost {
                    write!(f, " (temp cost {temporary_cost})")?;
                }
            }
        }
        Ok(())
    }
}

pub type CardRef = Rc<RefCell<Card>>;
pub type CardPile = Vec<CardRef>;
