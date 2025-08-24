use crate::{
    action::Action,
    actions::place_card_in_hand::PlaceCardInHandAction,
    cards::{CardCost, random_red_attack_in_combat},
    game::Game,
};

pub struct InfernalBladeAction();

impl Action for InfernalBladeAction {
    fn run(&self, game: &mut Game) {
        let class = random_red_attack_in_combat(&mut game.rng);
        let c = game.new_card(class);
        if let CardCost::Cost { temporary_cost, .. } = &mut c.borrow_mut().cost {
            *temporary_cost = Some(0);
        }
        game.action_queue.push_top(PlaceCardInHandAction(c));
    }
}

impl std::fmt::Debug for InfernalBladeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "infernal blade")
    }
}
