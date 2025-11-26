use crate::{action::Action, card::CardRef, game::Game};

pub struct UpgradeAllAction();

fn upgrade<'a, T: Iterator<Item = &'a CardRef>>(cards: T) {
    for c in cards {
        let mut c = c.borrow_mut();
        if c.can_upgrade() {
            c.upgrade();
        }
    }
}

impl Action for UpgradeAllAction {
    fn run(&self, game: &mut Game) {
        upgrade(game.hand.iter());
        upgrade(game.discard_pile.iter());
        upgrade(game.draw_pile.get_all().into_iter());
        upgrade(game.exhaust_pile.iter());
    }
}

impl std::fmt::Debug for UpgradeAllAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "upgrade all")
    }
}
