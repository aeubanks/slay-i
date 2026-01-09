use rand::Rng;

use crate::{action::Action, cards::CardType, game::Game, rng::rand_slice};

pub struct UpgradeTwoRandomInMasterAction(pub CardType);

impl Action for UpgradeTwoRandomInMasterAction {
    fn run(&self, game: &mut Game) {
        let mut cards = game
            .master_deck
            .iter()
            .filter(|c| {
                let c = c.borrow();
                c.class.ty() == self.0 && c.can_upgrade()
            })
            .collect::<Vec<_>>();
        if cards.len() > 2 {
            let i1 = game.rng.random_range(0..cards.len());
            let c1 = cards.remove(i1);
            let i2 = game.rng.random_range(0..cards.len());
            let c2 = cards.remove(i2);
            cards = vec![c1, c2];
        }
        for c in cards {
            c.borrow_mut().upgrade();
        }
    }
}

impl std::fmt::Debug for UpgradeTwoRandomInMasterAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "upgrade two {:?} random in master", self.0)
    }
}

pub struct UpgradeRandomInMasterAction;

impl Action for UpgradeRandomInMasterAction {
    fn run(&self, game: &mut Game) {
        let cards = game
            .master_deck
            .iter()
            .filter(|c| c.borrow().can_upgrade())
            .collect::<Vec<_>>();
        if !cards.is_empty() {
            rand_slice(&mut game.rng, &cards).borrow_mut().upgrade();
        }
    }
}

impl std::fmt::Debug for UpgradeRandomInMasterAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "upgrade random in master")
    }
}
