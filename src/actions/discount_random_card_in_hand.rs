use crate::{action::Action, card::CardRef, game::Game, rng::rand_slice};

pub struct DiscountRandomCardInHandAction();

fn can_discount(card: &CardRef) -> bool {
    match card.borrow().cost {
        crate::cards::CardCost::Cost {
            base_cost,
            temporary_cost,
            free_to_play_once,
        } => !free_to_play_once && temporary_cost.unwrap_or(base_cost) != 0,
        _ => false,
    }
}

impl Action for DiscountRandomCardInHandAction {
    fn run(&self, game: &mut Game) {
        let cards = game
            .hand
            .iter()
            .filter(|c| can_discount(c))
            .collect::<Vec<_>>();
        let c = match cards.len() {
            0 => return,
            1 => cards[0],
            _ => rand_slice(&mut game.rng, &cards),
        };
        c.borrow_mut().set_temporary_cost(0);
    }
}

impl std::fmt::Debug for DiscountRandomCardInHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "discount random card in hand")
    }
}
