use crate::{
    action::Action, actions::decrease_max_hp::DecreaseMaxHPAction, cards::CardClass, game::Game,
};

pub struct RemovedCardFromMasterDeckAction(pub CardClass);

impl Action for RemovedCardFromMasterDeckAction {
    fn run(&self, game: &mut Game) {
        if self.0 == CardClass::Parasite {
            game.action_queue.push_bot(DecreaseMaxHPAction(3));
        }
    }
}

impl std::fmt::Debug for RemovedCardFromMasterDeckAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "removed card from master deck {:?}", self.0)
    }
}
