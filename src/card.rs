use std::{cell::RefCell, rc::Rc};

use crate::{
    cards::{CardClass, CardCost, CardType},
    game::CreatureRef,
};

#[derive(Clone, Copy)]
pub struct CardPlayInfo {
    pub target: Option<CreatureRef>,
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
        if self.class.upgrade_removes_exhaust() {
            self.exhaust = false;
        }
        if let CardCost::Cost {
            base_cost,
            temporary_cost,
        } = &mut self.cost
        {
            if let Some(new_cost) = self.class.upgrade_cost(*base_cost) {
                let prev_base_cost = *base_cost;
                *base_cost = new_cost;
                // temporary cost gets adjusted the same amount
                if let Some(temp) = temporary_cost {
                    *temp += new_cost - prev_base_cost;
                    if *temp < 0 {
                        *temp = 0;
                    }
                }
            }
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
    pub fn is_ethereal(&self) -> bool {
        use CardClass::*;
        match self.class {
            GhostlyArmor | Dazed | AscendersBane | Clumsy | Carnage => true,
            Apparition => self.upgrade_count == 0,
            _ => false,
        }
    }
    pub fn has_target(&self) -> bool {
        use CardClass::*;
        match self.class {
            Strike | Bash | PommelStrike | TwinStrike | Clothesline | BodySlam | IronWave
            | WildStrike | Headbutt | PerfectedStrike | HeavyBlade | Anger | Clash
            | SearingBlow | Rampage | Uppercut | SeverSoul | Carnage | Hemokinesis | Dropkick
            | Pummel | BloodForBlood | RecklessCharge | SpotWeakness | Bludgeon | Feed
            | FiendFire | SwiftStrike | FlashOfSteel | MindBlast | Bite | RitualDagger
            | HandOfGreed | DebugKill => true,
            Blind | Trip => self.upgrade_count == 0,
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

#[cfg(test)]
mod tests {
    use crate::cards::{CardClass, CardCost, new_card};

    #[test]
    fn test_upgrade_temp_cost() {
        for (init_temp, final_temp) in [(3, 2), (2, 1), (1, 0), (0, 0)] {
            let c = new_card(CardClass::BodySlam);
            let mut c = c.borrow_mut();
            match &mut c.cost {
                CardCost::Cost {
                    base_cost: _,
                    temporary_cost,
                } => {
                    *temporary_cost = Some(init_temp);
                }
                _ => unreachable!(),
            }
            assert_eq!(
                c.cost,
                CardCost::Cost {
                    base_cost: 1,
                    temporary_cost: Some(init_temp)
                }
            );
            c.upgrade();
            assert_eq!(
                c.cost,
                CardCost::Cost {
                    base_cost: 0,
                    temporary_cost: Some(final_temp)
                }
            );
        }
    }
}
