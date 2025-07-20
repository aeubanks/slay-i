use crate::{action::Action, card::CardRef, game::Game};

pub struct UpgradeOneCardInHandAction(pub CardRef);

impl Action for UpgradeOneCardInHandAction {
    fn run(&self, _: &mut Game) {
        self.0.borrow_mut().upgrade();
    }
}

impl std::fmt::Debug for UpgradeOneCardInHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "upgrade {:?} in hand", self.0.borrow())
    }
}
