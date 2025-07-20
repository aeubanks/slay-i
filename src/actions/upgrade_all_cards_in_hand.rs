use crate::{action::Action, game::Game};

pub struct UpgradeAllCardsInHandAction();

impl Action for UpgradeAllCardsInHandAction {
    fn run(&self, game: &mut Game) {
        for c in &game.hand {
            let mut c = c.borrow_mut();
            if c.can_upgrade() {
                c.upgrade();
            }
        }
    }
}

impl std::fmt::Debug for UpgradeAllCardsInHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "upgrade all cards in hand")
    }
}
