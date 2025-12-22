use crate::{
    action::Action,
    actions::{gain_gold::GainGoldAction, increase_max_hp::IncreaseMaxHPAction},
    card::CardRef,
    cards::CardType,
    game::Game,
    relic::RelicClass,
};

pub struct AddCardToMasterDeckAction(pub CardRef);

impl Action for AddCardToMasterDeckAction {
    fn run(&self, game: &mut Game) {
        if self.0.borrow().class.ty() == CardType::Curse
            && let Some(v) = game.get_relic_value(RelicClass::Omamori)
            && v > 0
        {
            game.set_relic_value(RelicClass::Omamori, v - 1);
            return;
        }

        if self.0.borrow().class.ty() == CardType::Curse
            && game.has_relic(RelicClass::DarkstonePeriapt)
        {
            game.action_queue.push_bot(IncreaseMaxHPAction(6));
        }
        if game.has_relic(RelicClass::CeramicFish) {
            game.action_queue.push_bot(GainGoldAction(9));
        }

        game.master_deck.push(self.0.clone());
    }
}

impl std::fmt::Debug for AddCardToMasterDeckAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "add card to master deck {:?}", self.0)
    }
}
