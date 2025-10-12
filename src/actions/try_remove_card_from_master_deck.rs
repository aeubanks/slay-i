use crate::{
    action::Action, actions::removed_card_from_master_deck::RemovedCardFromMasterDeckAction,
    cards::CardClass, game::Game,
};

pub struct TryRemoveCardFromMasterDeckAction(pub CardClass);

impl Action for TryRemoveCardFromMasterDeckAction {
    fn run(&self, game: &mut Game) {
        if let Some(i) = game
            .master_deck
            .iter()
            .position(|c| c.borrow().class == self.0)
        {
            game.master_deck.remove(i);
            game.action_queue
                .push_top(RemovedCardFromMasterDeckAction(self.0));
        }
    }
}

impl std::fmt::Debug for TryRemoveCardFromMasterDeckAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "remove card from master deck {:?}", self.0)
    }
}
