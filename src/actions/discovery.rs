use crate::{
    action::Action,
    actions::place_card_in_hand::PlaceCardInHandAction,
    cards::{CardClass, CardCost},
    game::Game,
};

pub struct DiscoveryAction {
    pub class: CardClass,
    pub amount: i32,
    pub is_free: bool,
}

impl Action for DiscoveryAction {
    fn run(&self, game: &mut Game) {
        for _ in 0..self.amount {
            let c = game.new_card(self.class);
            if self.is_free
                && let CardCost::Cost { temporary_cost, .. } = &mut c.borrow_mut().cost
            {
                *temporary_cost = Some(0)
            }
            game.action_queue.push_top(PlaceCardInHandAction(c));
        }
    }
}

impl std::fmt::Debug for DiscoveryAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "discovery")
    }
}
