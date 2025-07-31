use crate::{
    actions::{damage::DamageAction, gain_energy::GainEnergyAction},
    game::{CreatureRef, Rand},
    queue::ActionQueue,
    rng::rand_slice,
};
use lazy_static::lazy_static;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PotionRarity {
    Common,
    Uncommon,
    Rare,
}

type PotionBehavior = fn(Option<CreatureRef>, &mut ActionQueue);

macro_rules! p {
    ($($name:ident => ($rarity:expr, $has_target:expr, $behavior:expr)),+,) => {
        #[allow(dead_code)]
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Potion {
            $(
                $name,
            )+
        }
        impl Potion {
            fn rarity(&self) -> PotionRarity {
                use PotionRarity::*;
                match self {
                    $(Self::$name => $rarity,)+
                }
            }
            pub fn behavior(&self) -> PotionBehavior {
                match self {
                    $(Self::$name => $behavior,)+
                }
            }
            pub fn has_target(&self) -> bool {
                match self {
                    $(Self::$name => $has_target,)+
                }
            }
        }
        impl Potion {
            pub fn all() -> Vec<Self> {
                vec![$(Self::$name,)+]
            }
        }
    };
}

p!(
    Blood => (Common, false, blood),
    Block => (Common, false, block),
    Dex => (Common, false, dex),
    Energy => (Common, false, energy),
    Explosive => (Common, false, explosive),
    Fire => (Common, true, fire),
    Strength => (Common, false, strength),
    Swift => (Common, false, swift),
    Weak => (Common, true, weak),
    Fear => (Common, true, fear),
    Attack => (Common, false, attack),
    Skill => (Common, false, skill),
    Power => (Common, false, power),
    Colorless => (Common, false, colorless),
    Flex => (Common, false, flex),
    Speed => (Common, false, speed),
    Forge => (Common, false, forge),

    Elixir => (Uncommon, false, elixir),
    Regen => (Uncommon, false, regen),
    Ancient => (Uncommon, false, ancient),
    Bronze => (Uncommon, false, bronze),
    Gamblers => (Uncommon, false, gamblers),
    Steel  => (Uncommon, false, steel),
    Duplication => (Uncommon, false, duplication),
    Chaos => (Uncommon, false, chaos),
    Memories => (Uncommon, false, memories),

    Iron => (Rare, false, iron),
    Cultist => (Rare, false, cultist),
    Fruit => (Rare, false, fruit),
    Snecko => (Rare, false, snecko),
    Fairy => (Rare, false, fairy),
    Smoke => (Rare, false, smoke),
    Entropic => (Rare, false, entropic),
);

fn blood(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn block(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn dex(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn energy(_: Option<CreatureRef>, queue: &mut ActionQueue) {
    queue.push_bot(GainEnergyAction(2));
}
fn explosive(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn fire(target: Option<CreatureRef>, queue: &mut ActionQueue) {
    queue.push_bot(DamageAction::thorns(20, target.unwrap()));
}
fn strength(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn swift(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn weak(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn fear(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn attack(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn skill(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn power(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn colorless(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn flex(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn speed(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn forge(_: Option<CreatureRef>, _: &mut ActionQueue) {}

fn elixir(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn regen(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn ancient(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn bronze(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn gamblers(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn steel(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn duplication(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn chaos(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn memories(_: Option<CreatureRef>, _: &mut ActionQueue) {}

fn iron(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn cultist(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn fruit(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn snecko(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn fairy(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn smoke(_: Option<CreatureRef>, _: &mut ActionQueue) {}
fn entropic(_: Option<CreatureRef>, _: &mut ActionQueue) {}

lazy_static! {
    static ref ALL_COMMON: Vec<Potion> = Potion::all()
        .into_iter()
        .filter(|r| r.rarity() == PotionRarity::Common)
        .collect();
}

pub fn random_common_potion(rng: &mut Rand) -> Potion {
    rand_slice(rng, &ALL_COMMON)
}

#[cfg(test)]
mod tests {
    use crate::game::{GameBuilder, Move};

    use super::*;

    #[test]
    fn test_fire() {
        let mut g = GameBuilder::default().build_combat();
        g.player.add_potion(Potion::Fire);
        let hp = g.monsters[0].creature.cur_hp;
        g.make_move(Move::UsePotion {
            potion_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 20);
    }
}
