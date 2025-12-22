use crate::{
    action::Action, actions::add_card_to_master_deck::AddCardToMasterDeckAction, cards::CardClass,
    game::Game,
};

pub struct AddCardClassToMasterDeckAction(pub CardClass);

impl Action for AddCardClassToMasterDeckAction {
    fn run(&self, game: &mut Game) {
        let c = game.new_card(self.0);
        game.action_queue.push_top(AddCardToMasterDeckAction(c));
    }
}

impl std::fmt::Debug for AddCardClassToMasterDeckAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "add card to master deck {:?}", self.0)
    }
}
