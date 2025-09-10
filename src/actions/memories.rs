use crate::{action::Action, card::CardRef, cards::CardCost, game::Game};

pub struct MemoriesAction(pub CardRef);

impl Action for MemoriesAction {
    fn run(&self, game: &mut Game) {
        assert!(!game.hand_is_full());
        let c = self.0.clone();
        if let CardCost::Cost { temporary_cost, .. } = &mut c.borrow_mut().cost {
            *temporary_cost = Some(0);
        }
        game.hand.push(c);
    }
}

impl std::fmt::Debug for MemoriesAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "memories {:?}", self.0.borrow())
    }
}
