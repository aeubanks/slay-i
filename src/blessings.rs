use crate::{
    cards::random_uncommon_colorless,
    game::{Game, GameState},
    potion::random_common_potion,
    relic::random_common_relic,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Blessing {
    GainMaxHPSmall,
    CommonRelic,
    TransformOne,
    RandomUncommonColorless,
    RandomPotion,
}

impl Blessing {
    pub fn run(&self, game: &mut Game) {
        use Blessing::*;
        match self {
            GainMaxHPSmall => {
                game.increase_max_hp(8);
            }
            CommonRelic => {
                let r = random_common_relic(&mut game.rng);
                game.add_relic(r);
            }
            TransformOne => {
                game.state = GameState::BlessingTransform;
            }
            RandomUncommonColorless => {
                let r = random_uncommon_colorless(&mut game.rng);
                game.add_card_to_master_deck(r);
            }
            RandomPotion => {
                let p = random_common_potion(&mut game.rng);
                game.add_potion(p);
            }
        }
    }
}
