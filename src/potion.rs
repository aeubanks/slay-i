use crate::{
    actions::{
        block::BlockAction, damage::DamageAction, damage_all_monsters::DamageAllMonstersAction,
        draw::DrawAction, gain_energy::GainEnergyAction, gain_status::GainStatusAction,
        heal::HealAction, upgrade_all_cards_in_hand::UpgradeAllCardsInHandAction,
    },
    game::{CreatureRef, Game, Rand},
    rng::rand_slice,
    status::Status,
};
use lazy_static::lazy_static;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PotionRarity {
    Common,
    Uncommon,
    Rare,
}

type PotionBehavior = fn(bool, Option<CreatureRef>, &mut Game);

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

fn blood(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let percent = if is_sacred { 40 } else { 20 };
    let amount = game.player.creature.max_hp as f32 * percent as f32 / 100.0;
    game.action_queue.push_top(HealAction {
        target: CreatureRef::player(),
        amount: amount as i32,
    });
}
fn block(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 24 } else { 12 };
    game.action_queue
        .push_bot(BlockAction::player_flat_amount(amount));
}
fn dex(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 4 } else { 2 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Dexterity,
        amount,
        target: CreatureRef::player(),
    });
}
fn energy(_: bool, _: Option<CreatureRef>, game: &mut Game) {
    game.action_queue.push_bot(GainEnergyAction(2));
}
fn explosive(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    game.action_queue
        .push_bot(DamageAllMonstersAction::thorns(if is_sacred {
            20
        } else {
            10
        }));
}
fn fire(is_sacred: bool, target: Option<CreatureRef>, game: &mut Game) {
    game.action_queue.push_bot(DamageAction::thorns_rupture(
        if is_sacred { 40 } else { 20 },
        target.unwrap(),
    ));
}
fn strength(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 4 } else { 2 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Strength,
        amount,
        target: CreatureRef::player(),
    });
}
fn swift(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 6 } else { 3 };
    game.action_queue.push_bot(DrawAction(amount));
}
fn weak(is_sacred: bool, target: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 6 } else { 3 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Weak,
        amount,
        target: target.unwrap(),
    });
}
fn fear(is_sacred: bool, target: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 6 } else { 3 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Vulnerable,
        amount,
        target: target.unwrap(),
    });
}
fn attack(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn skill(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn power(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn colorless(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn flex(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 10 } else { 5 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Strength,
        amount,
        target: CreatureRef::player(),
    });
    game.action_queue.push_bot(GainStatusAction {
        status: Status::LoseStrength,
        amount,
        target: CreatureRef::player(),
    });
}
fn speed(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 10 } else { 5 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Dexterity,
        amount,
        target: CreatureRef::player(),
    });
    game.action_queue.push_bot(GainStatusAction {
        status: Status::LoseDexterity,
        amount,
        target: CreatureRef::player(),
    });
}
fn forge(_: bool, _: Option<CreatureRef>, game: &mut Game) {
    game.action_queue.push_bot(UpgradeAllCardsInHandAction());
}

fn elixir(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn regen(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 10 } else { 5 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::RegenPlayer,
        amount,
        target: CreatureRef::player(),
    });
}
fn ancient(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 2 } else { 1 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Artifact,
        amount,
        target: CreatureRef::player(),
    });
}
fn bronze(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 6 } else { 3 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Thorns,
        amount,
        target: CreatureRef::player(),
    });
}
fn gamblers(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn steel(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn duplication(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 2 } else { 1 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Duplication,
        amount,
        target: CreatureRef::player(),
    });
}
fn chaos(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn memories(_: bool, _: Option<CreatureRef>, _: &mut Game) {}

fn iron(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn cultist(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn fruit(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn snecko(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn fairy(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn smoke(_: bool, _: Option<CreatureRef>, _: &mut Game) {}
fn entropic(_: bool, _: Option<CreatureRef>, _: &mut Game) {}

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
        cards::CardClass,
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

    #[test]
    fn test_speed() {
        let mut g = GameBuilder::default().build_combat();
        g.throw_potion(Potion::Speed, None);
        g.play_card(CardClass::Defend, None);
        assert_eq!(g.player.creature.block, 10);

        g.make_move(Move::EndTurn);
        g.player.add_relic(RelicClass::SacredBark);
        g.throw_potion(Potion::Speed, None);
        g.play_card(CardClass::Defend, None);
        assert_eq!(g.player.creature.block, 15);

        g.make_move(Move::EndTurn);
        g.play_card(CardClass::Defend, None);
        assert_eq!(g.player.creature.block, 5);
    }

    #[test]
    fn test_blood() {
        let mut g = GameBuilder::default().build_combat();
        g.player.creature.cur_hp = 10;
        g.player.creature.max_hp = 100;
        g.throw_potion(Potion::Blood, None);
        assert_eq!(g.player.creature.cur_hp, 10 + 20);
        g.player.add_relic(RelicClass::SacredBark);
        g.throw_potion(Potion::Blood, None);
        assert_eq!(g.player.creature.cur_hp, 10 + 20 + 40);
    }
}
