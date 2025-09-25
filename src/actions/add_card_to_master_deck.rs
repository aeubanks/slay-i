use crate::{
    action::Action,
    actions::increase_max_hp::IncreaseMaxHPAction,
    cards::{CardClass, CardType},
    game::Game,
    relic::RelicClass,
};

pub struct AddCardToMasterDeckAction(pub CardClass);

impl Action for AddCardToMasterDeckAction {
    fn run(&self, game: &mut Game) {
        if self.0.ty() == CardType::Curse
            && let Some(v) = game.get_relic_value(RelicClass::Omamori)
            && v > 0
        {
            game.set_relic_value(RelicClass::Omamori, v - 1);
            return;
        }

        if self.0.ty() == CardType::Curse && game.has_relic(RelicClass::DarkstonePeriapt) {
            game.action_queue.push_bot(IncreaseMaxHPAction(6));
        }

        let c = game.new_card(self.0);

        game.master_deck.push(c);
    }
}

impl std::fmt::Debug for AddCardToMasterDeckAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "add card to master deck {:?}", self.0)
    }
}
