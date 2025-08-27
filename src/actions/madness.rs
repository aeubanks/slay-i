use crate::{action::Action, card::CardRef, cards::CardCost, game::Game, rng::rand_slice};

pub struct MadnessAction();

fn temp_cost_is_zero(c: &CardRef) -> bool {
    if let CardCost::Cost { temporary_cost, .. } = c.borrow().cost {
        temporary_cost == Some(0)
    } else {
        unreachable!();
    }
}

impl Action for MadnessAction {
    fn run(&self, g: &mut Game) {
        // madness can make any card with base cost non-zero free, but prioritizes cards that don't have a temporary cost of zero
        let mut not_free = vec![];
        let mut not_free_and_not_temp_free = vec![];
        for c in &g.hand {
            if let CardCost::Cost { base_cost, .. } = c.borrow().cost
                && base_cost != 0
            {
                not_free.push(c);
                if !temp_cost_is_zero(c) {
                    not_free_and_not_temp_free.push(c);
                }
            }
        }
        if not_free.is_empty() {
            return;
        }
        let c = if not_free_and_not_temp_free.is_empty() {
            rand_slice(&mut g.rng, &not_free)
        } else {
            rand_slice(&mut g.rng, &not_free_and_not_temp_free)
        };
        match &mut c.borrow_mut().cost {
            CardCost::Cost {
                base_cost,
                temporary_cost,
                ..
            } => {
                *base_cost = 0;
                *temporary_cost = None;
            }
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Debug for MadnessAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "madness")
    }
}
