use crate::{action::Action, card::CardRef, game::Game};

pub struct IncreaseBaseAmountAction {
    pub card_id: u32,
    pub amount: i32,
    pub master: bool,
}

fn maybe_increase_base_amount(c: &CardRef, card_id: u32, amount: i32) {
    let mut c = c.borrow_mut();
    if c.id == card_id {
        c.base_increase += amount;
    }
}

impl Action for IncreaseBaseAmountAction {
    fn run(&self, g: &mut Game) {
        for c in &g.discard_pile {
            maybe_increase_base_amount(c, self.card_id, self.amount);
        }
        for c in &g.draw_pile {
            maybe_increase_base_amount(c, self.card_id, self.amount);
        }
        for c in &g.exhaust_pile {
            maybe_increase_base_amount(c, self.card_id, self.amount);
        }
        for c in &g.hand {
            maybe_increase_base_amount(c, self.card_id, self.amount);
        }
        if self.master {
            for c in &g.player.master_deck {
                maybe_increase_base_amount(c, self.card_id, self.amount);
            }
        }
    }
}

impl std::fmt::Debug for IncreaseBaseAmountAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "increase base amount by {}", self.amount)
    }
}
