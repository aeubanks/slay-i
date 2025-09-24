use crate::{
    action::Action,
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

        let c = game.new_card(self.0);

        game.master_deck.push(c);
    }
}

impl std::fmt::Debug for AddCardToMasterDeckAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "add card to master deck {:?}", self.0)
    }
}
