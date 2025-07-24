use crate::{
    cards::random_uncommon_colorless,
    game::{Game, GameState},
    relic::random_relic,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Blessing {
    GainMaxHPSmall,
    CommonRelic,
    TransformOne,
    RandomUncommonColorless,
}

impl Blessing {
    pub fn run(&self, game: &mut Game) {
        use Blessing::*;
        match self {
            GainMaxHPSmall => {
                game.increase_max_hp(8);
            }
            CommonRelic => {
                game.player.add_relic(random_relic(&mut game.rng));
            }
            TransformOne => {
                game.state = GameState::BlessingTransform;
            }
            RandomUncommonColorless => {
                let r = random_uncommon_colorless(&mut game.rng);
                game.add_card_to_master_deck(r);
            }
        }
    }
}
