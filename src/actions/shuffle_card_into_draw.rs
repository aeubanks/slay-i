use crate::{
    action::Action,
    cards::{CardClass, CardCost},
    game::Game,
};

pub struct ShuffleCardIntoDrawAction {
    pub class: CardClass,
    pub is_free: bool,
}

impl Action for ShuffleCardIntoDrawAction {
    fn run(&self, game: &mut Game) {
        let card = game.new_card(self.class);
        if self.is_free
            && let CardCost::Cost { base_cost, .. } = &mut card.borrow_mut().cost
        {
            *base_cost = 0
        }
        game.draw_pile.shuffle_in_one(card);
    }
}

impl std::fmt::Debug for ShuffleCardIntoDrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shuffle card into draw: {:?}", self.class)
    }
}
