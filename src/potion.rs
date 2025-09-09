use crate::{
    actions::{
        damage::DamageAction, damage_all_monsters::DamageAllMonstersAction,
        gain_energy::GainEnergyAction,
    },
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

type PotionBehavior = fn(bool, Option<CreatureRef>, &mut ActionQueue);

macro_rules! p {
    ($($name:ident => ($rarity:expr, $has_target:expr, $behavior:expr)),+,) => {
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

fn blood(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn block(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn dex(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn energy(_: bool, _: Option<CreatureRef>, queue: &mut ActionQueue) {
    queue.push_bot(GainEnergyAction(2));
}
fn explosive(is_sacred: bool, _: Option<CreatureRef>, queue: &mut ActionQueue) {
    queue.push_bot(DamageAllMonstersAction::thorns(if is_sacred {
        20
    } else {
        10
    }));
}
fn fire(is_sacred: bool, target: Option<CreatureRef>, queue: &mut ActionQueue) {
    queue.push_bot(DamageAction::thorns_rupture(
        if is_sacred { 40 } else { 20 },
        target.unwrap(),
    ));
}
fn strength(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn swift(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn weak(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn fear(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn attack(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn skill(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn power(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn colorless(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn flex(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn speed(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn forge(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}

fn elixir(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn regen(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn ancient(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn bronze(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn gamblers(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn steel(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn duplication(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn chaos(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn memories(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}

fn iron(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn cultist(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn fruit(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn snecko(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn fairy(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn smoke(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}
fn entropic(_: bool, _: Option<CreatureRef>, _: &mut ActionQueue) {}

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
    use crate::{
        game::{GameBuilder, Move},
        monsters::test::NoopMonster,
        relic::RelicClass,
    };

    use super::*;

    #[test]
    fn test_fire() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .build_combat();
        g.player.add_potion(Potion::Fire);
        let hp = g.monsters[0].creature.cur_hp;
        g.make_move(Move::UsePotion {
            potion_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 20);
        assert_eq!(g.monsters[1].creature.cur_hp, hp);

        g.player.add_relic(RelicClass::SacredBark);
        g.throw_potion(Potion::Fire, Some(CreatureRef::monster(1)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 20);
        assert_eq!(g.monsters[1].creature.cur_hp, hp - 40);
    }

    #[test]
    fn test_explosive() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .build_combat();
        g.player.add_potion(Potion::Explosive);
        let hp = g.monsters[0].creature.cur_hp;
        g.make_move(Move::UsePotion {
            potion_index: 0,
            target: None,
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 10);
        assert_eq!(g.monsters[1].creature.cur_hp, hp - 10);

        g.player.add_relic(RelicClass::SacredBark);
        g.throw_potion(Potion::Explosive, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 30);
        assert_eq!(g.monsters[1].creature.cur_hp, hp - 30);
    }
}
