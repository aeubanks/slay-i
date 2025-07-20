use crate::{game::Game, relics::random_relic};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Blessing {
    GainMaxHPSmall,
    CommonRelic,
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
        }
    }
}
