use crate::{
    action::Action, actions::place_card_in_hand::PlaceCardInHandAction, card::CardRef, game::Game,
};

pub struct DualWieldAction {
    pub card: CardRef,
    pub amount: i32,
    pub destroy_original: bool,
}

impl Action for DualWieldAction {
    fn run(&self, game: &mut Game) {
        if self.destroy_original {
            for _ in 0..(self.amount + 1) {
                let new_c = game.clone_card_new_id(&self.card);
                game.action_queue.push_top(PlaceCardInHandAction(new_c));
            }
        } else {
            for _ in 0..self.amount {
                let new_c = game.clone_card_new_id(&self.card);
                game.action_queue.push_top(PlaceCardInHandAction(new_c));
            }
            game.action_queue
                .push_top(PlaceCardInHandAction(self.card.clone()));
        }
    }
}

impl std::fmt::Debug for DualWieldAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dual wield {} {:?}", self.amount, self.card.borrow())
    }
}
