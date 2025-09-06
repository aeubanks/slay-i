use crate::{action::Action, card::CardRef, game::Game};

pub struct UpgradeAllAction();

impl Action for UpgradeAllAction {
    fn run(&self, game: &mut Game) {
        let upgrade = |cards: &[CardRef]| {
            for c in cards {
                let mut c = c.borrow_mut();
                if c.can_upgrade() {
                    c.upgrade();
                }
            }
        };
        upgrade(&game.hand);
        upgrade(&game.discard_pile);
        upgrade(&game.draw_pile);
        upgrade(&game.exhaust_pile);
    }
}

impl std::fmt::Debug for UpgradeAllAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "upgrade all")
    }
}
