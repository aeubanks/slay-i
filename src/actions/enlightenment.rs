use crate::{action::Action, cards::CardCost, game::Game};

pub struct EnlightenmentAction {
    pub for_combat: bool,
}

impl Action for EnlightenmentAction {
    fn run(&self, game: &mut Game) {
        for c in &game.hand {
            let mut c = c.borrow_mut();
            match &mut c.cost {
                CardCost::Cost {
                    base_cost,
                    temporary_cost,
                } => {
                    if temporary_cost.unwrap_or(*base_cost) > 1 {
                        if self.for_combat {
                            *base_cost = 1;
                        } else {
                            *temporary_cost = Some(1);
                        }
                    }
                }
                CardCost::X | CardCost::Zero => {}
            }
        }
    }
}

impl std::fmt::Debug for EnlightenmentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "enlightenment for {}",
            if self.for_combat { "combat" } else { "turn" }
        )
    }
}
