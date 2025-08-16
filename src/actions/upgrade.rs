use crate::{action::Action, card::CardRef, game::Game};

pub struct UpgradeAction(pub CardRef);

impl Action for UpgradeAction {
    fn run(&self, _: &mut Game) {
        let mut c = self.0.borrow_mut();
        assert!(c.can_upgrade());
        c.upgrade();
    }
}

impl std::fmt::Debug for UpgradeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "upgrade {:?} in hand", self.0.borrow())
    }
}
