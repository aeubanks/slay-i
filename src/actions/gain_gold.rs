use crate::{
    action::Action,
    actions::heal::HealAction,
    game::{CreatureRef, Game},
    relic::RelicClass,
};

pub struct GainGoldAction(pub i32);

impl Action for GainGoldAction {
    fn run(&self, game: &mut Game) {
        if game.has_relic(RelicClass::Ectoplasm) {
            return;
        }
        game.gold += self.0;
        if game.has_relic(RelicClass::BloodyIdol) {
            game.action_queue.push_top(HealAction {
                target: CreatureRef::player(),
                amount: 5,
            });
        }
    }
}

impl std::fmt::Debug for GainGoldAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gain gold {}", self.0)
    }
}
