use crate::{
    action::Action, actions::place_card_in_hand::PlaceCardInHandAction, cards::CardType,
    game::Game, rng::rand_slice,
};

pub struct ViolenceAction(pub i32);

impl Action for ViolenceAction {
    fn run(&self, game: &mut Game) {
        let mut cards = Vec::new();
        for _ in 0..self.0 {
            let attack_indexes = game
                .draw_pile
                .iter()
                .enumerate()
                .filter(|(_, c)| c.borrow().class.ty() == CardType::Attack)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            if attack_indexes.is_empty() {
                break;
            }
            cards.push(
                game.draw_pile
                    .remove(rand_slice(&mut game.rng, &attack_indexes)),
            );
        }
        while let Some(c) = cards.pop() {
            game.action_queue.push_top(PlaceCardInHandAction(c));
        }
    }
}

impl std::fmt::Debug for ViolenceAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "violence")
    }
}
