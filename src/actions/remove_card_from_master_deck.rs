use crate::{
    action::Action, actions::decrease_max_hp::DecreaseMaxHPAction, cards::CardClass, game::Game,
};

pub struct RemoveCardFromMasterDeckAction(pub CardClass);

impl Action for RemoveCardFromMasterDeckAction {
    fn run(&self, game: &mut Game) {
        if self.0 == CardClass::Parasite {
            game.action_queue.push_bot(DecreaseMaxHPAction(3));
        }
    }
}

impl std::fmt::Debug for RemoveCardFromMasterDeckAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "remove card from master deck {:?}", self.0)
    }
}
