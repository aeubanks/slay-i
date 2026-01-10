use rand::Rng;

use crate::{
    game::Game,
    monster::{Monster, MonsterBehavior},
    monsters::{
        blue_slaver::BlueSlaver, cultist::Cultist, fungi_beast::FungiBeast,
        gremlin_fat::GremlinFat, gremlin_mad::GremlinMad, gremlin_nob::GremlinNob,
        gremlin_shield::GremlinShield, gremlin_sneaky::GremlinSneaky,
        gremlin_wizard::GremlinWizard, guardian::Guardian, hexaghost::Hexaghost, jawworm::JawWorm,
        lagavulin::Lagavulin, looter::Looter, louse::Louse, red_slaver::RedSlaver, sentry::Sentry,
        slime_acid_l::SlimeAcidL, slime_acid_m::SlimeAcidM, slime_acid_s::SlimeAcidS,
        slime_boss::SlimeBoss, slime_spike_l::SlimeSpikeL, slime_spike_m::SlimeSpikeM,
        slime_spike_s::SlimeSpikeS,
    },
    rng::remove_random,
};

pub mod blue_slaver;
pub mod cultist;
pub mod fungi_beast;
pub mod gremlin_fat;
pub mod gremlin_mad;
pub mod gremlin_nob;
pub mod gremlin_shield;
pub mod gremlin_sneaky;
pub mod gremlin_wizard;
pub mod guardian;
pub mod hexaghost;
pub mod jawworm;
pub mod lagavulin;
pub mod looter;
pub mod louse;
pub mod red_slaver;
pub mod sentry;
pub mod slime_acid_l;
pub mod slime_acid_m;
pub mod slime_acid_s;
pub mod slime_boss;
pub mod slime_spike_l;
pub mod slime_spike_m;
pub mod slime_spike_s;
pub mod test;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Combat {
    // act 1 easy
    Cultist,
    JawWorm,
    TwoLouses,
    SmallSlimes,
    // act 1 hard
    BlueSlaver,
    GremlinGang,
    Looter,
    LargeSlime,
    LotsOfSlimes,
    ExordiumThugs,
    ExordiumWildlife,
    RedSlaver,
    ThreeLouses,
    TwoFungiBeasts,
    // act 1 elites
    GremlinNob,
    Lagavulin,
    ThreeSentries,
    // act 1 events
    ThreeFungiBeasts,
    LagavulinEvent,
    // act 1 bosses
    Guardian,
    Hexaghost,
    SlimeBoss,
    // act 2 easy
    SphericGuardian,
    Chosen,
    ShellParasite,
    ThreeByrds,
    TwoThieves,
    // act 2 hard
    ChosenAndByrd,
    ChosenAndCultist,
    SentryAndSphericGuardian,
    SnakePlant,
    Snecko,
    ThreeCultists,
    ShelledParasiteAndFungiBeast,
    CenturionAndMystic,
    // act 2 events
    MaskedBandits,
    TwoSlavers,
    SlaverAndGremlinNob,
    // act 2 elites
    GremlinLeader,
    Slavers,
    BookOfStabbing,
    // act 2 bosses
    Automaton,
    Collector,
    Champ,
    // act 3 easy
    OrbWalker,
    ThreeShapes,
    // act 3 easy and hard
    ThreeDarklings,
    // act 3 hard
    FourShapes,
    SpireGrowth,
    Transient,
    Maw,
    SphereAndTwoShapes,
    JawWormHorde,
    WrithingMass,
    // act 3 events
    TwoOrbWalkers,
    // act 3 elites
    GiantHead,
    Nemesis,
    Reptomancer,
    // act 3 bosses
    AwakenedOne,
    TimeEater,
    DonuDeca,
    // act 4 elite
    SpireShieldAndSpear,
    CorruptHeart,
}

struct Helper(Vec<Box<dyn MonsterBehavior>>);

impl Helper {
    fn add<M: MonsterBehavior + 'static>(&mut self, m: M) {
        self.0.push(Box::new(m));
    }
    fn add_boxed(&mut self, m: Box<dyn MonsterBehavior>) {
        self.0.push(m);
    }
}

impl Combat {
    pub fn monsters(&self, game: &mut Game) -> Vec<Monster> {
        let mut ret = Helper(vec![]);
        match self {
            Combat::Cultist => ret.add(Cultist::new()),
            Combat::JawWorm => ret.add(JawWorm::new()),
            Combat::TwoLouses => {
                for _ in 0..2 {
                    if game.rng.random() {
                        ret.add(Louse::green(&mut game.rng));
                    } else {
                        ret.add(Louse::red(&mut game.rng));
                    }
                }
            }
            Combat::SmallSlimes => {
                if game.rng.random() {
                    ret.add(SlimeSpikeS::new());
                    ret.add(SlimeAcidM::new());
                } else {
                    ret.add(SlimeAcidS::new());
                    ret.add(SlimeSpikeM::new());
                }
            }
            Combat::BlueSlaver => ret.add(BlueSlaver::new()),
            Combat::GremlinGang => {
                let mut pool = vec![
                    Box::new(GremlinFat::new()) as Box<dyn MonsterBehavior>,
                    Box::new(GremlinFat::new()),
                    Box::new(GremlinMad::new()),
                    Box::new(GremlinMad::new()),
                    Box::new(GremlinSneaky::new()),
                    Box::new(GremlinSneaky::new()),
                    Box::new(GremlinShield::new()),
                    Box::new(GremlinWizard::new()),
                ];
                for _ in 0..4 {
                    ret.add_boxed(remove_random(&mut game.rng, &mut pool));
                }
            }
            Combat::Looter => {
                ret.add(Looter::new());
            }
            Combat::LargeSlime => {
                if game.rng.random() {
                    ret.add(SlimeSpikeL::new());
                } else {
                    ret.add(SlimeAcidL::new());
                }
            }
            Combat::LotsOfSlimes => {
                let mut pool = vec![
                    Box::new(SlimeSpikeS::new()) as Box<dyn MonsterBehavior>,
                    Box::new(SlimeSpikeS::new()),
                    Box::new(SlimeSpikeS::new()),
                    Box::new(SlimeAcidS::new()),
                    Box::new(SlimeAcidS::new()),
                ];
                while !pool.is_empty() {
                    ret.add_boxed(remove_random(&mut game.rng, &mut pool));
                }
            }
            Combat::ExordiumThugs => {
                let mut pool1 = vec![
                    Box::new(SlimeSpikeM::new()) as Box<dyn MonsterBehavior>,
                    Box::new(SlimeAcidM::new()),
                ];
                if game.rng.random() {
                    pool1.push(Box::new(Louse::green(&mut game.rng)));
                } else {
                    pool1.push(Box::new(Louse::red(&mut game.rng)));
                }
                ret.add_boxed(remove_random(&mut game.rng, &mut pool1));

                let mut pool2 = vec![
                    Box::new(Looter::new()) as Box<dyn MonsterBehavior>,
                    Box::new(Cultist::new()),
                ];
                if game.rng.random() {
                    pool2.push(Box::new(RedSlaver::new()));
                } else {
                    pool2.push(Box::new(BlueSlaver::new()));
                }
                ret.add_boxed(remove_random(&mut game.rng, &mut pool2));
            }
            Combat::ExordiumWildlife => {
                let mut pool1 = vec![
                    Box::new(FungiBeast::new()) as Box<dyn MonsterBehavior>,
                    Box::new(JawWorm::new()),
                ];
                if game.rng.random() {
                    pool1.push(Box::new(Louse::green(&mut game.rng)));
                } else {
                    pool1.push(Box::new(Louse::red(&mut game.rng)));
                }
                ret.add_boxed(remove_random(&mut game.rng, &mut pool1));

                let mut pool2 = vec![
                    Box::new(SlimeSpikeM::new()) as Box<dyn MonsterBehavior>,
                    Box::new(SlimeAcidM::new()),
                ];
                if game.rng.random() {
                    pool2.push(Box::new(Louse::green(&mut game.rng)));
                } else {
                    pool2.push(Box::new(Louse::red(&mut game.rng)));
                }
                ret.add_boxed(remove_random(&mut game.rng, &mut pool2));
            }
            Combat::RedSlaver => ret.add(RedSlaver::new()),
            Combat::ThreeLouses => {
                for _ in 0..3 {
                    if game.rng.random() {
                        ret.add(Louse::green(&mut game.rng));
                    } else {
                        ret.add(Louse::red(&mut game.rng));
                    }
                }
            }
            Combat::TwoFungiBeasts => {
                ret.add(FungiBeast::new());
                ret.add(FungiBeast::new());
            }
            Combat::GremlinNob => ret.add(GremlinNob::new()),
            Combat::Lagavulin => ret.add(Lagavulin::new()),
            Combat::ThreeSentries => {
                ret.add(Sentry::new_debuff_first());
                ret.add(Sentry::new_attack_first());
                ret.add(Sentry::new_debuff_first());
            }
            Combat::ThreeFungiBeasts => {
                ret.add(FungiBeast::new());
                ret.add(FungiBeast::new());
                ret.add(FungiBeast::new());
            }
            Combat::LagavulinEvent => ret.add(Lagavulin::new_event()),
            Combat::Guardian => ret.add(Guardian::new()),
            Combat::Hexaghost => ret.add(Hexaghost::new()),
            Combat::SlimeBoss => ret.add(SlimeBoss::new()),
            _ => panic!(),
        }
        assert!(!ret.0.is_empty());
        ret.0
            .into_iter()
            .map(|m| Monster::new_boxed(m, &mut game.rng))
            .collect()
    }
}
