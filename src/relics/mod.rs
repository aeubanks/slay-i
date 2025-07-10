use rand::Rng;

use crate::{
    game::Rand,
    relic::Relic,
    relics::{bag_of_prep::BagOfPrep, blood_vial::BloodVial},
};

pub mod bag_of_prep;
pub mod blood_vial;
pub mod burning_blood;

pub fn random_relic(rng: &mut Rand) -> Box<dyn Relic> {
    fn b<R: Relic + 'static>(r: R) -> Box<dyn Relic> {
        Box::new(r)
    }
    let relics = [b(BagOfPrep()), b(BloodVial())];
    let i = rng.random_range(0..relics.len());
    relics.into_iter().nth(i).unwrap()
}
