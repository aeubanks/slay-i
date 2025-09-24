use crate::{
    actions::{
        add_card_to_master_deck::AddCardToMasterDeckAction, increase_max_hp::IncreaseMaxHPAction,
    },
    cards::random_uncommon_colorless,
    game::Game,
    potion::random_common_potion,
    relic::random_common_relic,
    state::GameState,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Blessing {
    GainMaxHPSmall,
    CommonRelic,
    RemoveRelic, // just to prevent dead code warnings
    TransformOne,
    RemoveOne,
    RandomUncommonColorless,
    RandomPotion,
}

impl Blessing {
    pub fn run(&self, game: &mut Game) {
        use Blessing::*;
        match self {
            GainMaxHPSmall => {
                game.action_queue.push_bot(IncreaseMaxHPAction(8));
            }
            CommonRelic => {
                let r = random_common_relic(&mut game.rng);
                game.add_relic(r);
            }
            RemoveRelic => {
                game.remove_relic(game.relics[0].get_class());
            }
            TransformOne => {
                game.state.push_state(GameState::TransformCard);
            }
            RemoveOne => {
                game.state.push_state(GameState::RemoveCard);
            }
            RandomUncommonColorless => {
                let r = random_uncommon_colorless(&mut game.rng);
                game.action_queue.push_bot(AddCardToMasterDeckAction(r));
            }
            RandomPotion => {
                let p = random_common_potion(&mut game.rng);
                game.add_potion(p);
            }
        }
    }
}
