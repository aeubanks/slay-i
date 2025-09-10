use crate::{
    actions::{
        block::BlockAction,
        choose_cards_in_hand_to_exhaust::ChooseCardsInHandToExhaustAction,
        choose_discovery::{ChooseDiscoveryAction, ChooseDiscoveryType},
        choose_gamble::ChooseGambleAction,
        choose_memories::ChooseMemoriesAction,
        damage::DamageAction,
        damage_all_monsters::DamageAllMonstersAction,
        draw::DrawAction,
        gain_energy::GainEnergyAction,
        gain_status::GainStatusAction,
        heal::HealAction,
        increase_max_hp::IncreaseMaxHPAction,
        play_top_card::PlayTopCardAction,
        randomize_hand_cost::RandomizeHandCostAction,
        upgrade_all_cards_in_hand::UpgradeAllCardsInHandAction,
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
    Steel => (Uncommon, false, steel),
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
fn attack(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 2 } else { 1 };
    game.action_queue.push_bot(ChooseDiscoveryAction {
        ty: ChooseDiscoveryType::RedAttack,
        amount,
    });
}
fn skill(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 2 } else { 1 };
    game.action_queue.push_bot(ChooseDiscoveryAction {
        ty: ChooseDiscoveryType::RedSkill,
        amount,
    });
}
fn power(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 2 } else { 1 };
    game.action_queue.push_bot(ChooseDiscoveryAction {
        ty: ChooseDiscoveryType::RedPower,
        amount,
    });
}
fn colorless(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 2 } else { 1 };
    game.action_queue.push_bot(ChooseDiscoveryAction {
        ty: ChooseDiscoveryType::Colorless,
        amount,
    });
}
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

fn elixir(_: bool, _: Option<CreatureRef>, game: &mut Game) {
    game.action_queue
        .push_bot(ChooseCardsInHandToExhaustAction(10));
}
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
fn gamblers(_: bool, _: Option<CreatureRef>, game: &mut Game) {
    game.action_queue.push_bot(ChooseGambleAction());
}
fn steel(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 8 } else { 4 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::PlatedArmor,
        amount,
        target: CreatureRef::player(),
    });
}
fn duplication(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 2 } else { 1 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Duplication,
        amount,
        target: CreatureRef::player(),
    });
}
fn chaos(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 6 } else { 3 };
    for _ in 0..amount {
        game.action_queue.push_bot(PlayTopCardAction {
            force_exhaust: false,
        });
    }
}
fn memories(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 2 } else { 1 };
    game.action_queue.push_bot(ChooseMemoriesAction(amount));
}

fn iron(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 12 } else { 6 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Metallicize,
        amount,
        target: CreatureRef::player(),
    });
}
fn cultist(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 2 } else { 1 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Ritual,
        amount,
        target: CreatureRef::player(),
    });
}
fn fruit(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 10 } else { 5 };
    game.action_queue.push_bot(IncreaseMaxHPAction(amount));
}
fn snecko(is_sacred: bool, _: Option<CreatureRef>, game: &mut Game) {
    let amount = if is_sacred { 10 } else { 5 };
    game.action_queue.push_bot(DrawAction(amount));
    game.action_queue.push_bot(RandomizeHandCostAction());
}
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
        assert_matches,
        cards::{CardClass, CardColor, CardCost, CardType, random_red_in_combat},
        game::{GameBuilder, GameStatus, Move},
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

    #[test]
    fn test_attack() {
        let mut g = GameBuilder::default().build_combat();
        g.throw_potion(Potion::Attack, None);
        g.make_move(g.valid_moves()[0]);
        assert_eq!(g.hand.len(), 1);
        g.player.add_relic(RelicClass::SacredBark);
        g.throw_potion(Potion::Attack, None);
        g.make_move(g.valid_moves()[0]);
        assert_eq!(g.hand.len(), 3);

        for c in &g.hand {
            let c = c.borrow();
            assert_ne!(c.class, CardClass::Reaper);
            assert_eq!(c.class.ty(), CardType::Attack);
            assert_eq!(c.class.color(), CardColor::Red);
            if let CardCost::Cost { temporary_cost, .. } = c.cost {
                assert_eq!(temporary_cost, Some(0));
            }
        }
    }

    #[test]
    fn test_skill() {
        let mut g = GameBuilder::default().build_combat();
        g.throw_potion(Potion::Skill, None);
        g.make_move(g.valid_moves()[0]);
        assert_eq!(g.hand.len(), 1);
        g.player.add_relic(RelicClass::SacredBark);
        g.throw_potion(Potion::Skill, None);
        g.make_move(g.valid_moves()[0]);
        assert_eq!(g.hand.len(), 3);

        for c in &g.hand {
            let c = c.borrow();
            assert_eq!(c.class.ty(), CardType::Skill);
            assert_eq!(c.class.color(), CardColor::Red);
            if let CardCost::Cost { temporary_cost, .. } = c.cost {
                assert_eq!(temporary_cost, Some(0));
            }
        }
    }

    #[test]
    fn test_power() {
        let mut g = GameBuilder::default().build_combat();
        g.throw_potion(Potion::Power, None);
        g.make_move(g.valid_moves()[0]);
        assert_eq!(g.hand.len(), 1);
        g.player.add_relic(RelicClass::SacredBark);
        g.throw_potion(Potion::Power, None);
        g.make_move(g.valid_moves()[0]);
        assert_eq!(g.hand.len(), 3);

        for c in &g.hand {
            let c = c.borrow();
            assert_eq!(c.class.ty(), CardType::Power);
            assert_eq!(c.class.color(), CardColor::Red);
            if let CardCost::Cost { temporary_cost, .. } = c.cost {
                assert_eq!(temporary_cost, Some(0));
            }
        }
    }

    #[test]
    fn test_elixir() {
        let mut g = GameBuilder::default().build_combat();
        for _ in 0..10 {
            g.add_card_to_hand(CardClass::Strike);
        }
        g.throw_potion(Potion::Elixir, None);
        for _ in 0..10 {
            g.make_move(Move::ExhaustCardsInHand { card_index: 0 });
        }
        assert_eq!(g.hand.len(), 0);
        assert_eq!(g.exhaust_pile.len(), 10);

        g.clear_all_piles();
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::Strike);
        g.throw_potion(Potion::Elixir, None);
        g.make_move(Move::ExhaustCardsInHand { card_index: 0 });
        g.make_move(Move::ExhaustCardsInHandEnd);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 1);
    }

    #[test]
    fn test_chaos() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_draw_pile(CardClass::PerfectedStrike);
        g.add_card_to_draw_pile(CardClass::PerfectedStrike);
        g.add_card_to_discard_pile(CardClass::PerfectedStrike);
        let hp = g.monsters[0].creature.cur_hp;
        g.throw_potion(Potion::Chaos, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6 - 8 - 10);

        g.player.add_relic(RelicClass::SacredBark);
        for _ in 0..10 {
            g.add_card_to_draw_pile(CardClass::Defend);
        }
        g.throw_potion(Potion::Chaos, None);
        assert_eq!(g.player.creature.block, 6 * 5);

        assert_eq!(g.energy, 3);
    }

    #[test]
    fn test_gamble() {
        let mut g = GameBuilder::default().build_combat();
        g.throw_potion(Potion::Gamblers, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 0);

        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_draw_pile(CardClass::Inflame);
        g.throw_potion(Potion::Gamblers, None);
        g.make_move(Move::GambleEnd);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 2);
        g.throw_potion(Potion::Gamblers, None);
        g.make_move(Move::Gamble { card_index: 0 });
        g.make_move(Move::GambleEnd);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 2);
        assert_eq!(g.hand[0].borrow().class, CardClass::Defend);
        assert_eq!(g.hand[1].borrow().class, CardClass::Inflame);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.discard_pile[0].borrow().class, CardClass::Strike);
    }

    #[test]
    fn test_snecko_oil() {
        let mut g = GameBuilder::default().build_combat();
        let c = g.new_card(CardClass::Strike);
        c.borrow_mut().set_cost(1, Some(2));
        c.borrow_mut().set_free_to_play_once();
        g.draw_pile.push(c);
        g.throw_potion(Potion::Snecko, None);
        assert_eq!(g.hand.len(), 1);
        if let CardCost::Cost {
            base_cost,
            temporary_cost,
            free_to_play_once,
        } = g.hand[0].borrow().cost
        {
            assert!(base_cost >= 0 && base_cost <= 3);
            assert!(temporary_cost.is_none());
            assert!(free_to_play_once);
        } else {
            panic!();
        }
        g.clear_all_piles();
        g.player.add_relic(RelicClass::SacredBark);
        let mut found_cost = [false; 4];
        for _ in 0..100 {
            g.clear_all_piles();
            for _ in 0..11 {
                let class = random_red_in_combat(&mut g.rng);
                g.add_card_to_draw_pile(class);
            }
            g.throw_potion(Potion::Snecko, None);
            assert_eq!(g.hand.len(), 10);
            for c in &g.hand {
                match c.borrow().cost {
                    CardCost::Cost {
                        base_cost,
                        temporary_cost,
                        free_to_play_once,
                    } => {
                        assert!(base_cost >= 0 && base_cost <= 3);
                        assert!(temporary_cost.is_none());
                        assert!(!free_to_play_once);
                        found_cost[base_cost as usize] = true;
                    }
                    CardCost::X | CardCost::Zero => {}
                }
            }
            if found_cost.iter().all(|b| *b) {
                break;
            }
        }
        assert!(found_cost.iter().all(|b| *b));
    }

    #[test]
    fn test_memories() {
        let mut g = GameBuilder::default().build_combat();

        g.throw_potion(Potion::Memories, None);
        assert_eq!(g.hand.len(), 0);
        assert_eq!(g.discard_pile.len(), 0);

        g.add_card_to_discard_pile(CardClass::Strike);
        g.throw_potion(Potion::Memories, None);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().get_temporary_cost(), Some(0));
        assert_eq!(g.discard_pile.len(), 0);

        g.clear_all_piles();
        g.add_card_to_discard_pile(CardClass::Strike);
        g.add_card_to_discard_pile(CardClass::Defend);
        g.throw_potion(Potion::Memories, None);
        g.make_move(Move::Memories { card_index: 1 });
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Defend);
        assert_eq!(g.hand[0].borrow().get_temporary_cost(), Some(0));
        assert_eq!(g.discard_pile.len(), 1);

        g.clear_all_piles();
        g.add_card_to_discard_pile(CardClass::Strike);
        g.add_card_to_discard_pile(CardClass::Defend);
        g.throw_potion(Potion::Memories, None);
        g.make_move(Move::Memories { card_index: 1 });
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Defend);
        assert_eq!(g.hand[0].borrow().get_temporary_cost(), Some(0));
        assert_eq!(g.discard_pile.len(), 1);

        g.clear_all_piles();
        g.add_card_to_discard_pile(CardClass::Strike);
        for _ in 0..10 {
            g.add_card_to_hand(CardClass::Strike);
        }
        g.throw_potion(Potion::Memories, None);
        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.discard_pile.len(), 1);

        g.player.add_relic(RelicClass::SacredBark);
        g.clear_all_piles();
        g.add_card_to_discard_pile(CardClass::Defend);
        g.add_card_to_discard_pile(CardClass::Defend);
        for _ in 0..9 {
            g.add_card_to_hand(CardClass::Strike);
        }
        g.throw_potion(Potion::Memories, None);
        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.hand[9].borrow().class, CardClass::Defend);
        assert_eq!(g.hand[9].borrow().get_temporary_cost(), Some(0));
        assert_eq!(g.discard_pile.len(), 1);

        g.clear_all_piles();
        g.add_card_to_discard_pile(CardClass::Defend);
        g.add_card_to_discard_pile(CardClass::Defend);
        for _ in 0..8 {
            g.add_card_to_hand(CardClass::Strike);
        }
        g.throw_potion(Potion::Memories, None);
        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.hand[8].borrow().class, CardClass::Defend);
        assert_eq!(g.hand[8].borrow().get_temporary_cost(), Some(0));
        assert_eq!(g.hand[9].borrow().class, CardClass::Defend);
        assert_eq!(g.hand[9].borrow().get_temporary_cost(), Some(0));
        assert_eq!(g.discard_pile.len(), 0);

        g.clear_all_piles();
        g.add_card_to_discard_pile(CardClass::Defend);
        g.add_card_to_discard_pile(CardClass::FlameBarrier);
        g.add_card_to_discard_pile(CardClass::Inflame);
        for _ in 0..8 {
            g.add_card_to_hand(CardClass::Strike);
        }
        g.throw_potion(Potion::Memories, None);
        g.make_move(Move::Memories { card_index: 1 });
        g.make_move(Move::Memories { card_index: 1 });
        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.hand[8].borrow().class, CardClass::FlameBarrier);
        assert_eq!(g.hand[8].borrow().get_temporary_cost(), Some(0));
        assert_eq!(g.hand[9].borrow().class, CardClass::Inflame);
        assert_eq!(g.hand[9].borrow().get_temporary_cost(), Some(0));
        assert_eq!(g.discard_pile.len(), 1);

        g.clear_all_piles();
        g.add_card_to_discard_pile(CardClass::Defend);
        g.add_card_to_discard_pile(CardClass::FlameBarrier);
        g.add_card_to_discard_pile(CardClass::Inflame);
        for _ in 0..9 {
            g.add_card_to_hand(CardClass::Strike);
        }
        g.throw_potion(Potion::Memories, None);
        g.make_move(Move::Memories { card_index: 1 });
        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.hand[9].borrow().class, CardClass::FlameBarrier);
        assert_eq!(g.hand[9].borrow().get_temporary_cost(), Some(0));
        assert_eq!(g.discard_pile.len(), 2);
    }
}
