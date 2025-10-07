use rand::Rng;

use crate::{action::Action, game::Game};

pub struct UpgradeRandomInHandAction();

impl Action for UpgradeRandomInHandAction {
    fn run(&self, game: &mut Game) {
        let cards = game
            .hand
            .iter()
            .filter(|c| c.borrow().can_upgrade())
            .collect::<Vec<_>>();
        match cards.len() {
            0 => {}
            1 => {
                cards[0].borrow_mut().upgrade();
            }
            _ => {
                let i = game.rng.random_range(0..cards.len());
                cards[i].borrow_mut().upgrade();
            }
        }
    }
}

impl std::fmt::Debug for UpgradeRandomInHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "upgrade random in hand")
    }
}
