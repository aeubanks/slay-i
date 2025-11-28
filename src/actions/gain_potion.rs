use crate::{action::Action, game::Game, potion::Potion, relic::RelicClass};

pub struct GainPotionAction(pub Potion);

impl Action for GainPotionAction {
    fn run(&self, game: &mut Game) {
        if game.has_relic(RelicClass::Sozu) {
            return;
        }
        let mut added = false;
        for p in &mut game.potions {
            if p.is_none() {
                *p = Some(self.0);
                added = true;
                break;
            }
        }
        assert!(added);
    }
}

impl std::fmt::Debug for GainPotionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gain potion {:?}", self.0)
    }
}
