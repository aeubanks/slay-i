use crate::{
    action::Action,
    game::Game,
    relic::{RelicClass, new_relic},
};

pub struct GainRelicAction(pub RelicClass);

impl Action for GainRelicAction {
    fn run(&self, game: &mut Game) {
        let mut r = new_relic(self.0);
        r.on_equip(&mut game.action_queue);
        game.relics.push(r);
    }
}

impl std::fmt::Debug for GainRelicAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gain relic {:?}", self.0)
    }
}
