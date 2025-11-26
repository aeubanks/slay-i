use crate::{
    actions::{
        choose_card_in_discard_to_place_on_top_of_draw::ChooseCardInDiscardToPlaceOnTopOfDrawAction,
        damage::{DamageAction, OnFatal, OnFatalType},
        damage_all_monsters::DamageAllMonstersAction,
        damage_random_monster::DamageRandomMonsterAction,
        discard_card::DiscardCardAction,
        draw::DrawAction,
        dropkick::DropkickAction,
        exhaust_non_attack_in_hand::ExhaustNonAttackInHandAction,
        fiend_fire::FiendFireAction,
        gain_status::GainStatusAction,
        gain_status_all_monsters::GainStatusAllMonstersAction,
        heal::HealAction,
        increase_base_amount::IncreaseBaseAmountAction,
        shuffle_card_into_draw::ShuffleCardIntoDrawAction,
        vampire::VampireAction,
    },
    card::{CardPlayInfo, CardRef},
    cards::{CardClass, skills::push_block},
    game::{CreatureRef, Game},
    relic::RelicClass,
    status::Status,
};

fn extra_base_damage(game: &Game, info: &CardPlayInfo) -> i32 {
    if game.has_relic(RelicClass::StrikeDummy) && info.card.class.is_strike() {
        3
    } else {
        0
    }
}

fn push_damage(
    game: &mut Game,
    info: &CardPlayInfo,
    unupgraded_base_damage: i32,
    upgraded_base_damage: i32,
) {
    game.action_queue.push_bot(DamageAction::from_player(
        if info.upgraded {
            upgraded_base_damage
        } else {
            unupgraded_base_damage
        } + extra_base_damage(game, info),
        info.target.unwrap(),
    ));
}

fn push_aoe_damage(
    game: &mut Game,
    info: &CardPlayInfo,
    unupgraded_base_damage: i32,
    upgraded_base_damage: i32,
) {
    game.action_queue
        .push_bot(DamageAllMonstersAction::from_player(
            if info.upgraded {
                upgraded_base_damage
            } else {
                unupgraded_base_damage
            } + extra_base_damage(game, info),
        ));
}

pub fn strike_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 6, 9);
}

pub fn bash_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 8, 10);
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Vulnerable,
        amount: if info.upgraded { 3 } else { 2 },
        target: info.target.unwrap(),
    });
}

pub fn pommel_strike_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 9, 10);
    game.action_queue
        .push_bot(DrawAction(if info.upgraded { 2 } else { 1 }));
}

pub fn twin_strike_behavior(game: &mut Game, info: &CardPlayInfo) {
    for _ in 0..2 {
        push_damage(game, info, 5, 7);
    }
}

pub fn clothesline_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 12, 14);
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Weak,
        amount: if info.upgraded { 3 } else { 2 },
        target: info.target.unwrap(),
    });
}

pub fn cleave_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_aoe_damage(game, info, 8, 11);
}

pub fn thunderclap_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_aoe_damage(game, info, 4, 7);
    game.action_queue.push_bot(GainStatusAllMonstersAction {
        status: Status::Vulnerable,
        amount: 1,
    });
}

pub fn body_slam_behavior(game: &mut Game, info: &CardPlayInfo) {
    let damage = game.player.block;
    push_damage(game, info, damage, damage);
}

pub fn iron_wave_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 5, 7);
    push_damage(game, info, 5, 7);
}

pub fn wild_strike_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 12, 17);
    game.action_queue.push_bot(ShuffleCardIntoDrawAction {
        class: CardClass::Wound,
        is_free: false,
    });
}

pub fn headbutt_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 9, 12);
    game.action_queue
        .push_bot(ChooseCardInDiscardToPlaceOnTopOfDrawAction());
}

pub fn sword_boomerang_behavior(game: &mut Game, info: &CardPlayInfo) {
    let count = if info.upgraded { 4 } else { 3 };
    for _ in 0..count {
        game.action_queue.push_bot(DamageRandomMonsterAction {
            amount: 3,
            thorns: false,
        });
    }
}

fn count_strikes<'a, T: Iterator<Item = &'a CardRef>>(cards: T) -> i32 {
    cards.filter(|c| c.borrow().class.is_strike()).count() as i32
}

pub fn perfected_strike_behavior(game: &mut Game, info: &CardPlayInfo) {
    let num_strikes = count_strikes(game.hand.iter())
        + count_strikes(game.discard_pile.iter())
        + count_strikes(game.draw_pile.get_all().into_iter());
    let base = 6 + num_strikes * if info.upgraded { 3 } else { 2 };
    push_damage(game, info, base, base);
}

pub fn heavy_blade_behavior(game: &mut Game, info: &CardPlayInfo) {
    let strength = game.player.get_status(Status::Strength).unwrap_or(0);
    push_damage(game, info, 14 + strength * 2, 14 + strength * 4);
}

pub fn anger_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 6, 8);
    let card = game.clone_card_new_id(info.card);
    game.action_queue.push_bot(DiscardCardAction(card));
}

pub fn clash_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 14, 18);
}

pub fn reckless_charge_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 7, 10);
    game.action_queue.push_bot(ShuffleCardIntoDrawAction {
        class: CardClass::Dazed,
        is_free: false,
    });
}

pub fn searing_blow_behavior(game: &mut Game, info: &CardPlayInfo) {
    let n = info.upgrade_count;
    game.action_queue.push_bot(DamageAction::from_player(
        n * (n + 7) / 2 + 12,
        info.target.unwrap(),
    ));
}

pub fn whirlwind_behavior(game: &mut Game, info: &CardPlayInfo) {
    let mut count = info.cost;
    if game.has_relic(RelicClass::ChemicalX) {
        count += 2;
    }
    for _ in 0..count {
        push_aoe_damage(game, info, 5, 8);
    }
}

pub fn rampage_behavior(game: &mut Game, info: &CardPlayInfo) {
    let damage = 8 + info.base_increase;
    push_damage(game, info, damage, damage);
    game.action_queue.push_bot(IncreaseBaseAmountAction {
        card_id: info.card.id,
        amount: if info.upgraded { 8 } else { 5 },
        master: false,
    });
}

pub fn uppercut_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 13, 13);
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Weak,
        amount: if info.upgraded { 2 } else { 1 },
        target: info.target.unwrap(),
    });
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Vulnerable,
        amount: if info.upgraded { 2 } else { 1 },
        target: info.target.unwrap(),
    });
}

pub fn sever_soul_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(ExhaustNonAttackInHandAction());
    push_damage(game, info, 16, 22);
}

pub fn carnage_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 20, 28);
}

pub fn hemokinesis_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(DamageAction::lose_hp(2, CreatureRef::player()));
    push_damage(game, info, 15, 20);
}

pub fn dropkick_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(DropkickAction(info.target.unwrap()));
    push_damage(game, info, 5, 8);
}

pub fn pummel_behavior(game: &mut Game, info: &CardPlayInfo) {
    let count = if info.upgraded { 5 } else { 4 };
    for _ in 0..count {
        push_damage(game, info, 2, 2);
    }
}

pub fn blood_for_blood_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 18, 22);
}

pub fn reaper_behavior(game: &mut Game, info: &CardPlayInfo) {
    let alive = game
        .monsters
        .iter()
        .enumerate()
        .filter(|(_, m)| m.creature.is_alive())
        .map(|(i, _)| CreatureRef::monster(i))
        .collect::<Vec<_>>();
    push_aoe_damage(game, info, 4, 5);
    game.action_queue.push_bot(VampireAction(alive));
}

pub fn immolate_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_aoe_damage(game, info, 21, 28);
    let card = game.new_card(CardClass::Burn);
    game.action_queue.push_bot(DiscardCardAction(card));
}

pub fn bludgeon_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 32, 42);
}

pub fn feed_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(DamageAction::from_player_with_on_fatal(
            if info.upgraded { 12 } else { 10 },
            info.target.unwrap(),
            OnFatal {
                ty: OnFatalType::Feed,
                upgraded: info.upgraded,
            },
        ));
}

pub fn fiend_fire_behavior(game: &mut Game, info: &CardPlayInfo) {
    let amount = if info.upgraded { 10 } else { 7 };
    game.action_queue.push_bot(FiendFireAction {
        target: info.target.unwrap(),
        amount,
    });
}

pub fn swift_strike_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 7, 10);
}

pub fn flash_of_steel_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 3, 6);
    game.action_queue.push_bot(DrawAction(1));
}

pub fn dramatic_entrance_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_aoe_damage(game, info, 8, 12);
}

pub fn mind_blast_behavior(game: &mut Game, info: &CardPlayInfo) {
    let damage = game.draw_pile.len() as i32;
    push_damage(game, info, damage, damage);
}

pub fn bite_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 7, 8);
    game.action_queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: if info.upgraded { 3 } else { 2 },
    });
}

pub fn ritual_dagger_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(DamageAction::from_player_with_on_fatal(
            15 + info.base_increase,
            info.target.unwrap(),
            OnFatal {
                ty: OnFatalType::RitualDagger {
                    card_id: info.card.id,
                },
                upgraded: info.upgraded,
            },
        ));
}

pub fn hand_of_greed_behavior(game: &mut Game, info: &CardPlayInfo) {
    let base_amount = if info.upgraded { 25 } else { 20 };
    game.action_queue
        .push_bot(DamageAction::from_player_with_on_fatal(
            base_amount,
            info.target.unwrap(),
            OnFatal {
                ty: OnFatalType::HandOfGreed,
                upgraded: info.upgraded,
            },
        ));
}

pub fn debug_kill_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_damage(game, info, 9999, 9999);
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::block::BlockAction,
        assert_matches,
        cards::{CardClass, CardCost},
        game::{CreatureRef, Game, GameBuilder, GameStatus, Move},
        monster::Monster,
        monsters::test::{AttackMonster, NoopMonster},
        status::Status,
    };

    #[test]
    fn test_strike() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Strike)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6);
    }

    #[test]
    fn test_upgraded_strike() {
        let mut g = GameBuilder::default().build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card_upgraded(CardClass::Strike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 9);
    }

    #[test]
    fn test_bash() {
        let mut g = GameBuilder::default().build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::Bash, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 8);
        assert_eq!(
            g.monsters[0].creature.get_status(Status::Vulnerable),
            Some(2)
        );
    }

    #[test]
    fn test_pommel_strike() {
        let mut gb = GameBuilder::default();
        for _ in 0..10 {
            gb = gb.add_card(CardClass::PommelStrike);
        }
        let mut g = gb.build_combat();
        assert_eq!(g.draw_pile.len(), 5);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.hand.len(), 5);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.draw_pile.len(), 4);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.hand.len(), 5);
    }

    #[test]
    fn test_pommel_strike_upgrade() {
        let mut gb = GameBuilder::default();
        for _ in 0..10 {
            gb = gb.add_card_upgraded(CardClass::PommelStrike);
        }
        let mut g = gb.build_combat();
        assert_eq!(g.draw_pile.len(), 5);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.hand.len(), 5);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.draw_pile.len(), 3);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.hand.len(), 6);
    }

    #[test]
    fn test_cleave() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Cleave, 2)
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .build_combat();
        let hp0 = g.monsters[0].creature.cur_hp;
        g.monsters[1].creature.cur_hp = 4;
        g.play_card(CardClass::Cleave, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8);
        assert_eq!(g.monsters[1].creature.cur_hp, 0);
        g.play_card(CardClass::Cleave, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 16);
        assert_eq!(g.monsters[1].creature.cur_hp, 0);
    }

    #[test]
    fn test_thunderclap() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .build_combat();
        let hp0 = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::Thunderclap, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 4);
        assert_eq!(g.monsters[1].creature.cur_hp, hp0 - 4);
        g.play_card(CardClass::Thunderclap, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 10);
        assert_eq!(g.monsters[1].creature.cur_hp, hp0 - 10);
    }

    #[test]
    fn test_body_slam() {
        let mut g = GameBuilder::default().build_combat();
        g.run_action(BlockAction::player_flat_amount(5));
        let hp0 = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::BodySlam, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 5);
    }

    #[test]
    fn test_wild_strike() {
        let mut g = GameBuilder::default().build_combat();
        let hp0 = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::WildStrike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 12);
        assert_eq!(g.draw_pile.len(), 1);
        assert_eq!(g.draw_pile.pop(&mut g.rng).borrow().class, CardClass::Wound);
    }

    #[test]
    fn test_headbutt() {
        let mut g = GameBuilder::default().build_combat();
        g.energy = 10;
        g.play_card(CardClass::Headbutt, Some(CreatureRef::monster(0)));
        assert_eq!(g.draw_pile.len(), 0);
        assert_eq!(g.discard_pile.len(), 1);

        g.play_card(CardClass::Headbutt, Some(CreatureRef::monster(0)));
        assert_eq!(g.draw_pile.len(), 1);
        assert_eq!(g.discard_pile.len(), 1);

        g.add_card_to_discard_pile(CardClass::Strike);
        g.play_card(CardClass::Headbutt, Some(CreatureRef::monster(0)));
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::PlaceCardInDiscardOnTopOfDraw { card_index: 0 },
                Move::PlaceCardInDiscardOnTopOfDraw { card_index: 1 },
            ]
        );
        g.make_move(Move::PlaceCardInDiscardOnTopOfDraw { card_index: 1 });
        assert_eq!(g.draw_pile.len(), 2);
        assert_eq!(
            g.draw_pile.pop(&mut g.rng).borrow().class,
            CardClass::Strike
        );
        assert_eq!(
            g.draw_pile.pop(&mut g.rng).borrow().class,
            CardClass::Headbutt
        );
    }

    #[test]
    fn test_sword_boomerang() {
        {
            let mut g = GameBuilder::default()
                .add_monster_status(Status::Thorns, 1)
                .add_monster_status(Status::Vulnerable, 1)
                .build_combat();
            g.player.cur_hp = 50;
            g.monsters[0].creature.cur_hp = 20;
            g.play_card(CardClass::SwordBoomerang, None);
            assert_eq!(g.monsters[0].creature.cur_hp, 8);
            assert_eq!(g.player.cur_hp, 47);
        }
        let mut found_3_0 = false;
        let mut found_2_1 = false;
        let mut found_1_2 = false;
        let mut found_0_3 = false;
        for _ in 0..500 {
            let mut g = GameBuilder::default()
                .add_monster(NoopMonster::with_hp(50))
                .add_monster(NoopMonster::with_hp(50))
                .add_monster(NoopMonster::with_hp(50))
                .build_combat();
            g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(1)));
            g.play_card(CardClass::SwordBoomerang, None);
            match (g.monsters[0].creature.cur_hp, g.monsters[2].creature.cur_hp) {
                (41, 50) => found_3_0 = true,
                (44, 47) => found_2_1 = true,
                (47, 44) => found_1_2 = true,
                (50, 41) => found_0_3 = true,
                _ => panic!(),
            }
            if found_3_0 && found_2_1 && found_1_2 && found_0_3 {
                break;
            }
        }
        assert!(found_3_0 && found_2_1 && found_1_2 && found_0_3);
    }

    #[test]
    fn test_perfected_strike() {
        let mut g = GameBuilder::default().build_combat();
        g.energy = 99;

        g.monsters[0].creature.cur_hp = 100;
        g.play_card(CardClass::PerfectedStrike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 100 - 6);

        g.monsters[0].creature.cur_hp = 100;
        g.play_card(CardClass::PerfectedStrike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 100 - 6 - 2);

        g.monsters[0].creature.cur_hp = 100;
        g.add_card_to_draw_pile(CardClass::Strike);
        g.add_card_to_draw_pile(CardClass::Anger);
        g.play_card(CardClass::PerfectedStrike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 100 - 6 - 2 * 3);

        g.monsters[0].creature.cur_hp = 100;
        g.add_card_to_hand_upgraded(CardClass::TwinStrike);
        g.add_card_to_hand_upgraded(CardClass::Defend);
        g.play_card_upgraded(CardClass::PerfectedStrike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 100 - 6 - 3 * 5);
    }

    #[test]
    fn test_heavy_blade() {
        let mut g = GameBuilder::default().build_combat();
        g.energy = 99;

        g.monsters[0].creature.cur_hp = 100;
        g.play_card(CardClass::HeavyBlade, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 100 - 14);

        g.player.set_status(Status::Strength, 2);
        g.monsters[0].creature.cur_hp = 100;
        g.play_card(CardClass::HeavyBlade, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 100 - 14 - 2 * 3);

        g.player.set_status(Status::Strength, 2);
        g.monsters[0].creature.cur_hp = 100;
        g.play_card_upgraded(CardClass::HeavyBlade, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 100 - 14 - 2 * 5);

        g.player.set_status(Status::Strength, 10);
        g.monsters[0].creature.set_status(Status::Vulnerable, 1);
        g.monsters[0].creature.cur_hp = 100;
        g.play_card_upgraded(CardClass::HeavyBlade, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 100 - 21 - 75);
    }

    #[test]
    fn test_anger() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.discard_pile.len(), 2);
        assert_eq!(g.discard_pile[0].borrow().class, CardClass::Anger);
        assert_eq!(g.discard_pile[1].borrow().class, CardClass::Anger);

        let c = g.new_card(CardClass::Anger);
        c.borrow_mut().cost = CardCost::Cost {
            base_cost: 2,
            temporary_cost: Some(1),
            free_to_play_once: false,
        };
        g.hand.push(c);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.discard_pile.len(), 4);
        assert_eq!(g.discard_pile[2].borrow().class, CardClass::Anger);
        assert_matches!(
            g.discard_pile[2].borrow().cost,
            CardCost::Cost {
                base_cost: 2,
                temporary_cost: None,
                free_to_play_once: false
            }
        );
        assert_eq!(g.discard_pile[3].borrow().class, CardClass::Anger);
        assert_matches!(
            g.discard_pile[3].borrow().cost,
            CardCost::Cost {
                base_cost: 2,
                temporary_cost: None,
                free_to_play_once: false
            }
        );
    }

    #[test]
    fn test_clash() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Clash);
        assert_eq!(g.valid_moves().len(), 2);
        g.add_card_to_hand(CardClass::Anger);
        assert_eq!(g.valid_moves().len(), 3);
        g.add_card_to_hand(CardClass::Defend);
        assert_eq!(g.valid_moves().len(), 3);
    }

    #[test]
    fn test_searing_blow() {
        for (upgrade_count, damage) in [(0, 12), (1, 16), (2, 21), (3, 27)] {
            let mut g = GameBuilder::default()
                .add_card(CardClass::SearingBlow)
                .build_combat();
            for _ in 0..upgrade_count {
                g.hand[0].borrow_mut().upgrade();
            }
            let hp = g.monsters[0].creature.cur_hp;
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: Some(0),
            });
            assert_eq!(g.monsters[0].creature.cur_hp, hp - damage);
        }
    }

    #[test]
    fn test_whirlwind() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Whirlwind, 2)
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .build_combat();

        let hp0 = g.monsters[0].creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 15);
        assert_eq!(g.monsters[1].creature.cur_hp, hp0 - 15);
        assert_eq!(g.energy, 0);

        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 15);
        assert_eq!(g.monsters[1].creature.cur_hp, hp0 - 15);
    }

    #[test]
    fn test_rampage() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Rampage)
            .build_combat();

        let hp0 = g.monsters[0].creature.cur_hp;

        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8);

        let c = g.discard_pile.pop().unwrap();
        g.hand.push(c);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8 - 13);

        let c = g.discard_pile.pop().unwrap();
        g.hand.push(c);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8 - 13 - 18);
    }

    #[test]
    fn test_rampage_upgraded() {
        let mut g = GameBuilder::default()
            .add_card_upgraded(CardClass::Rampage)
            .build_combat();
        let hp0 = g.monsters[0].creature.cur_hp;

        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8);

        let c = g.discard_pile.pop().unwrap();
        g.hand.push(c);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8 - 16);

        let c = g.discard_pile.pop().unwrap();
        g.hand.push(c);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8 - 16 - 24);
    }

    #[test]
    fn test_rampage_unupgraded_and_upgraded() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Rampage, 2)
            .build_combat();

        g.energy = 10;

        let hp0 = g.monsters[0].creature.cur_hp;

        g.make_move(Move::PlayCard {
            card_index: 1,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8);

        let c = g.discard_pile.pop().unwrap();
        c.borrow_mut().upgrade();
        g.hand.push(c);
        g.make_move(Move::PlayCard {
            card_index: 1,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8 - 13);

        let c = g.discard_pile.pop().unwrap();
        g.hand.push(c);
        g.make_move(Move::PlayCard {
            card_index: 1,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8 - 13 - 21);

        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8 - 13 - 21 - 8);
    }

    #[test]
    fn test_sever_soul() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::Wound);
        g.add_card_to_hand(CardClass::AscendersBane);
        g.play_card(CardClass::SeverSoul, Some(CreatureRef::monster(0)));
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Strike);
        assert_eq!(g.exhaust_pile.len(), 3);
        assert_eq!(g.exhaust_pile[0].borrow().class, CardClass::Defend);
        assert_eq!(g.exhaust_pile[1].borrow().class, CardClass::Wound);
        assert_eq!(g.exhaust_pile[2].borrow().class, CardClass::AscendersBane);
    }

    #[test]
    fn test_dropkick() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_draw_pile(CardClass::Strike);

        g.play_card(CardClass::Dropkick, Some(CreatureRef::monster(0)));
        assert_eq!(g.energy, 2);
        assert_eq!(g.hand.len(), 0);

        g.monsters[0].creature.set_status(Status::Vulnerable, 1);
        g.play_card(CardClass::Dropkick, Some(CreatureRef::monster(0)));
        assert_eq!(g.energy, 2);
        assert_eq!(g.hand.len(), 1);
    }

    #[test]
    fn test_dropkick_infinite() {
        {
            let mut g = GameBuilder::default()
                .add_cards(CardClass::Dropkick, 2)
                .build_combat();

            g.monsters[0].creature.set_status(Status::Vulnerable, 1);

            while !matches!(g.result(), GameStatus::Victory) {
                g.make_move(Move::PlayCard {
                    card_index: 0,
                    target: Some(0),
                });
            }
        }
        {
            let mut g = GameBuilder::default()
                .add_card(CardClass::DoubleTap)
                .add_card(CardClass::Dropkick)
                .build_combat();

            g.monsters[0].creature.set_status(Status::Vulnerable, 1);

            while !matches!(g.result(), GameStatus::Victory) {
                let double_tap_card_index = g
                    .hand
                    .iter()
                    .position(|c| c.borrow().class == CardClass::DoubleTap);
                if let Some(i) = double_tap_card_index {
                    g.make_move(Move::PlayCard {
                        card_index: i,
                        target: None,
                    });
                } else {
                    g.make_move(Move::PlayCard {
                        card_index: 0,
                        target: Some(0),
                    });
                }
            }
        }
    }

    #[test]
    fn test_blood_for_blood() {
        let mut g = GameBuilder::default()
            .add_monster(AttackMonster::new(1))
            .build_combat();
        g.add_card_to_discard_pile(CardClass::BloodForBlood);
        g.add_card_to_hand(CardClass::BloodForBlood);
        g.add_card_to_exhaust_pile(CardClass::BloodForBlood);
        assert_eq!(g.discard_pile[0].borrow().get_base_cost(), 4);
        assert_eq!(g.hand[0].borrow().get_base_cost(), 4);
        assert_eq!(g.exhaust_pile[0].borrow().get_base_cost(), 4);
        g.play_card(CardClass::Bloodletting, None);
        g.add_card_to_draw_pile(CardClass::BloodForBlood);
        assert_eq!(g.draw_pile.get(0).borrow().get_base_cost(), 3);
        assert_eq!(g.discard_pile[0].borrow().get_base_cost(), 3);
        assert_eq!(g.hand[0].borrow().get_base_cost(), 3);
        assert_eq!(g.exhaust_pile[0].borrow().get_base_cost(), 4);

        let cost_sum =
            |g: &Game| -> i32 { g.hand.iter().map(|c| c.borrow().get_base_cost()).sum() };

        g.player.block = 2;
        g.make_move(Move::EndTurn);
        assert_eq!(cost_sum(&g), 3 + 3 + 3);

        g.make_move(Move::EndTurn);
        assert_eq!(cost_sum(&g), 2 + 2 + 2);
        g.play_card_upgraded(CardClass::Armaments, None);
        assert_eq!(cost_sum(&g), 1 + 1 + 1);
        g.play_card(CardClass::Bloodletting, None);
        assert_eq!(cost_sum(&g), 0);
        g.play_card(CardClass::Bloodletting, None);
        assert_eq!(cost_sum(&g), 0);

        assert_eq!(g.exhaust_pile[0].borrow().get_base_cost(), 4);
    }

    #[test]
    fn test_blood_for_blood_multiple_combats() {
        let mut g = GameBuilder::default().build_combat();
        g.combat_monsters_queue
            .push(vec![Monster::new(NoopMonster::new(), &mut g.rng)]);
        g.play_card(CardClass::Bloodletting, None);
        g.play_card(CardClass::Bloodletting, None);
        g.add_card_to_hand(CardClass::BloodForBlood);
        assert_eq!(g.hand[0].borrow().get_base_cost(), 2);
        g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(0)));
        g.add_card_to_hand(CardClass::BloodForBlood);
        assert_eq!(g.hand[0].borrow().get_base_cost(), 4);
    }

    #[test]
    fn test_reaper() {
        {
            let mut g = GameBuilder::default().build_combat();
            g.player.cur_hp = 10;
            g.play_card(CardClass::Reaper, None);
            assert_eq!(g.player.cur_hp, 14);
        }
        {
            let mut g = GameBuilder::default().build_combat();
            g.player.cur_hp = 10;
            g.monsters[0].creature.cur_hp = 2;
            g.play_card(CardClass::Reaper, None);
            assert_eq!(g.player.cur_hp, 12);
        }
        {
            let mut g = GameBuilder::default().build_combat();
            g.player.cur_hp = 10;
            g.monsters[0].creature.block = 1;
            g.play_card(CardClass::Reaper, None);
            assert_eq!(g.player.cur_hp, 13);
        }
        {
            let mut g = GameBuilder::default()
                .add_monster_status(Status::Vulnerable, 1)
                .add_player_status(Status::Strength, 10)
                .build_combat();
            g.player.cur_hp = 10;
            g.play_card(CardClass::Reaper, None);
            assert_eq!(g.player.cur_hp, 31);
        }
        {
            let mut g = GameBuilder::default()
                .add_monster(NoopMonster::new())
                .add_monster(NoopMonster::new())
                .build_combat();
            g.player.cur_hp = 10;
            g.play_card(CardClass::Reaper, None);
            assert_eq!(g.player.cur_hp, 18);
        }
        {
            let mut g = GameBuilder::default()
                .add_monster(NoopMonster::new())
                .add_monster(NoopMonster::new())
                .build_combat();
            g.player.cur_hp = 10;
            g.monsters[0].creature.cur_hp = 1;
            g.monsters[1].creature.block = 1;
            g.play_card(CardClass::Reaper, None);
            assert_eq!(g.player.cur_hp, 14);
        }
        {
            let mut g = GameBuilder::default()
                .add_monster(NoopMonster::new())
                .add_monster(NoopMonster::new())
                .build_combat();
            g.player.cur_hp = 10;
            g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(0)));
            g.play_card(CardClass::Reaper, None);
            assert_eq!(g.player.cur_hp, 14);
        }
    }

    #[test]
    fn test_feed() {
        let mut g = GameBuilder::default().build_combat();
        let player_max_hp = g.player.max_hp;
        let player_cur_hp = g.player.cur_hp;
        let monster_hp = g.monsters[0].creature.cur_hp;

        g.play_card(CardClass::Feed, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, monster_hp - 10);
        assert_eq!(g.player.max_hp, player_max_hp);
        assert_eq!(g.player.cur_hp, player_cur_hp);

        g.monsters[0].creature.cur_hp = 8;
        g.play_card(CardClass::Feed, Some(CreatureRef::monster(0)));
        assert_matches!(g.result(), GameStatus::Victory);
        assert_eq!(g.player.max_hp, player_max_hp + 3);
        assert_eq!(g.player.cur_hp, player_cur_hp + 3);
    }

    #[test]
    fn test_feed_upgrade() {
        let mut g = GameBuilder::default().build_combat();
        let player_max_hp = g.player.max_hp;
        let player_cur_hp = g.player.cur_hp;
        let monster_hp = g.monsters[0].creature.cur_hp;

        g.play_card_upgraded(CardClass::Feed, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, monster_hp - 12);
        assert_eq!(g.player.max_hp, player_max_hp);
        assert_eq!(g.player.cur_hp, player_cur_hp);

        g.monsters[0].creature.cur_hp = 11;
        g.play_card_upgraded(CardClass::Feed, Some(CreatureRef::monster(0)));
        assert_matches!(g.result(), GameStatus::Victory);
        // assert_eq!(g.monsters[0].creature.cur_hp, 0);
        assert_eq!(g.player.max_hp, player_max_hp + 4);
        assert_eq!(g.player.cur_hp, player_cur_hp + 4);
    }
    #[test]
    fn test_fiend_fire() {
        let mut g = GameBuilder::default().build_combat();
        g.energy = 99;

        g.monsters[0].creature.cur_hp = 100;
        g.play_card(CardClass::FiendFire, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 100);

        g.monsters[0].creature.cur_hp = 100;
        g.exhaust_pile.clear();
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::Wound);
        g.add_card_to_hand(CardClass::AscendersBane);
        g.play_card(CardClass::FiendFire, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 100 - 7 * 4);
        assert_eq!(g.hand.len(), 0);
        assert_eq!(g.exhaust_pile.len(), 5);
        assert_eq!(g.exhaust_pile[0].borrow().class, CardClass::Defend);
        assert_eq!(g.exhaust_pile[1].borrow().class, CardClass::Strike);
        assert_eq!(g.exhaust_pile[2].borrow().class, CardClass::Wound);
        assert_eq!(g.exhaust_pile[3].borrow().class, CardClass::AscendersBane);
        assert_eq!(g.exhaust_pile[4].borrow().class, CardClass::FiendFire);
    }

    #[test]
    fn test_mind_blast() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 25)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.add_card_to_hand(CardClass::MindBlast);
        g.make_move(Move::PlayCard {
            card_index: 5,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 20);
    }

    #[test]
    fn test_ritual_dagger() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .add_cards(CardClass::RitualDagger, 2)
            .build_combat();

        g.energy = 10;

        let hp0 = g.monsters[0].creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 15);
        assert_eq!(g.master_deck[0].borrow().base_increase, 0);
        assert_eq!(g.master_deck[1].borrow().base_increase, 0);

        g.monsters[0].creature.cur_hp = 10;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.exhaust_pile[0].borrow().base_increase, 0);
        assert_eq!(g.exhaust_pile[1].borrow().base_increase, 3);
        assert!(
            g.master_deck[0].borrow().base_increase == 0
                || g.master_deck[1].borrow().base_increase == 0
        );
        assert!(
            g.master_deck[0].borrow().base_increase == 3
                || g.master_deck[1].borrow().base_increase == 3
        );

        let hp1 = g.monsters[1].creature.cur_hp;
        let c = g.exhaust_pile.pop().unwrap();
        g.hand.push(c);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(1),
        });
        assert_eq!(g.monsters[1].creature.cur_hp, hp1 - 18);

        g.monsters[1].creature.cur_hp = 17;
        let c = g.exhaust_pile.pop().unwrap();
        c.borrow_mut().upgrade();
        g.hand.push(c);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(1),
        });

        assert!(
            g.master_deck[0].borrow().base_increase == 0
                || g.master_deck[1].borrow().base_increase == 0
        );
        assert!(
            g.master_deck[0].borrow().base_increase == 8
                || g.master_deck[1].borrow().base_increase == 8
        );
    }

    #[test]
    fn test_hand_of_greed() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .build_combat();
        g.energy = 99;

        g.play_card(CardClass::HandOfGreed, Some(CreatureRef::monster(0)));
        assert_eq!(g.gold, 0);

        g.monsters[0].creature.cur_hp = 20;
        g.play_card(CardClass::HandOfGreed, Some(CreatureRef::monster(0)));
        assert_eq!(g.gold, 20);

        g.play_card(CardClass::HandOfGreed, Some(CreatureRef::monster(0)));
        assert_eq!(g.gold, 20);

        g.play_card_upgraded(CardClass::HandOfGreed, Some(CreatureRef::monster(1)));
        assert_eq!(g.gold, 20);

        g.monsters[1].creature.cur_hp = 24;
        g.play_card_upgraded(CardClass::HandOfGreed, Some(CreatureRef::monster(1)));
        assert_eq!(g.gold, 45);
    }

    #[test]
    fn test_debug_kill() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(0)));
        assert_matches!(g.result(), GameStatus::Victory);
    }

    #[test]
    fn test_flash_of_steel_finesse_infinite() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Finesse)
            .add_card(CardClass::FlashOfSteel)
            .add_monster(NoopMonster::new())
            .build_combat();

        for _ in 0..50 {
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: Some(0),
            });
        }
    }

    #[test]
    #[should_panic]
    fn test_upgrade_crash() {
        let mut g = GameBuilder::default().build_combat();
        let c = g.new_card_upgraded(CardClass::Strike);
        let mut c = c.borrow_mut();
        c.upgrade();
    }
}
