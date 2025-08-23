use crate::{action::Action, card::CardRef, cards::CardCost, game::Game};

pub struct ForethoughtAction(pub CardRef);

impl Action for ForethoughtAction {
    fn run(&self, game: &mut Game) {
        if let CardCost::Cost {
            free_to_play_once, ..
        } = &mut self.0.borrow_mut().cost
        {
            *free_to_play_once = true;
        }
        game.draw_pile.insert(0, self.0.clone());
    }
}

impl std::fmt::Debug for ForethoughtAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "forethought {:?}", self.0.borrow())
    }
}
