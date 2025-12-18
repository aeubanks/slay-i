use crate::{action::Action, game::Game, relic::RelicClass};

pub struct RemoveRelicAction(pub RelicClass);

impl Action for RemoveRelicAction {
    fn run(&self, game: &mut Game) {
        let idx = game.relics.iter().position(|r| r.get_class() == self.0);
        let mut r = game.relics.remove(idx.unwrap());
        r.on_unequip(&mut game.action_queue);
    }
}

impl std::fmt::Debug for RemoveRelicAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "remove relic {:?}", self.0)
    }
}
