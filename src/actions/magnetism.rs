use crate::{
    action::Action, actions::place_card_in_hand::PlaceCardInHandAction, cards::random_colorless,
    game::Game,
};

pub struct MagnetismAction();

impl Action for MagnetismAction {
    fn run(&self, game: &mut Game) {
        let class = random_colorless(&mut game.rng);
        let c = game.new_card(class);
        game.action_queue.push_top(PlaceCardInHandAction(c));
    }
}

impl std::fmt::Debug for MagnetismAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "magnetism")
    }
}
