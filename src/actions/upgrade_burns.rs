use crate::{action::Action, card::CardRef, cards::CardClass, game::Game};

pub struct UpgradeBurnsAction();

impl Action for UpgradeBurnsAction {
    fn run(&self, game: &mut Game) {
        let upgrade_burn = |c: &CardRef| {
            let mut c = c.borrow_mut();
            if c.class == CardClass::Burn {
                c.class = CardClass::BurnPlus;
            }
        };
        for c in &game.discard_pile {
            upgrade_burn(c);
        }
        for c in game.draw_pile.get_all() {
            upgrade_burn(c);
        }
    }
}

impl std::fmt::Debug for UpgradeBurnsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "upgrade burns")
    }
}
