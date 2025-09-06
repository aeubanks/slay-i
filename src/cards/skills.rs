use crate::{
    actions::{
        armaments::ArmamentsAction, block::BlockAction,
        block_per_non_attack_in_hand::BlockPerNonAttackInHandAction,
        choose_card_in_draw_to_place_in_hand::ChooseCardInDrawToPlaceInHandAction,
        choose_card_in_hand_to_exhaust::ChooseCardInHandToExhaustAction,
        choose_card_in_hand_to_place_on_top_of_draw::ChooseCardInHandToPlaceOnTopOfDrawAction,
        choose_cards_in_hand_to_exhaust::ChooseCardsInHandToExhaustAction,
        choose_discovery::ChooseDiscoveryAction, choose_dual_wield::ChooseDualWieldAction,
        choose_forethought_any::ChooseForethoughtAnyAction,
        choose_forethought_one::ChooseForethoughtOneAction, damage::DamageAction,
        double_block::DoubleBlockAction, double_strength::DoubleStrengthAction, draw::DrawAction,
        enlightenment::EnlightenmentAction,
        exhaust_non_attack_in_hand::ExhaustNonAttackInHandAction,
        exhaust_random_card_in_hand::ExhaustRandomCardInHandAction, exhume::ExhumeAction,
        gain_energy::GainEnergyAction, gain_status::GainStatusAction,
        gain_status_all_monsters::GainStatusAllMonstersAction, heal::HealAction,
        impatience::ImpatienceAction, infernal_blade::InfernalBladeAction, madness::MadnessAction,
        place_card_in_hand::PlaceCardInHandAction, play_top_card::PlayTopCardAction,
        shuffle_card_into_draw::ShuffleCardIntoDrawAction, spot_weakness::SpotWeaknessAction,
        upgrade_all::UpgradeAllAction, upgrade_all_cards_in_hand::UpgradeAllCardsInHandAction,
    },
    card::CardPlayInfo,
    cards::{
        CardClass, CardCost, CardType, random_colorless, random_red_attack_in_combat,
        random_red_skill_in_combat,
    },
    game::{CreatureRef, Game},
    status::Status,
};

pub fn push_block(
    game: &mut Game,
    info: &CardPlayInfo,
    unupgraded_base_block: i32,
    upgraded_base_block: i32,
) {
    game.action_queue
        .push_bot(BlockAction::player_card(if info.upgraded {
            upgraded_base_block
        } else {
            unupgraded_base_block
        }));
}

pub fn defend_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 5, 8);
}

pub fn armaments_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 5, 5);
    if info.upgraded {
        game.action_queue.push_bot(UpgradeAllCardsInHandAction());
    } else {
        game.action_queue.push_bot(ArmamentsAction());
    }
}

pub fn flex_behavior(game: &mut Game, info: &CardPlayInfo) {
    let amount = if info.upgraded { 4 } else { 2 };
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

pub fn true_grit_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 7, 9);
    if info.upgraded {
        game.action_queue
            .push_bot(ChooseCardInHandToExhaustAction());
    } else {
        game.action_queue.push_bot(ExhaustRandomCardInHandAction());
    }
}

pub fn shrug_it_off_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 8, 11);
    game.action_queue.push_bot(DrawAction(1));
}

pub fn havoc_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(PlayTopCardAction {
        force_exhaust: true,
    });
}

pub fn warcry_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(DrawAction(if info.upgraded { 2 } else { 1 }));
    game.action_queue
        .push_bot(ChooseCardInHandToPlaceOnTopOfDrawAction());
}

pub fn ghostly_armor_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 10, 13);
}

pub fn bloodletting_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(DamageAction::lose_hp(3, CreatureRef::player()));
    game.action_queue
        .push_bot(GainEnergyAction(if info.upgraded { 3 } else { 2 }));
}

pub fn sentinel_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 5, 8);
}

pub fn spot_weakness_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(SpotWeaknessAction {
        target: info.target.unwrap(),
        amount: if info.upgraded { 4 } else { 3 },
    });
}

pub fn dual_wield_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(ChooseDualWieldAction(if info.upgraded { 2 } else { 1 }));
}

pub fn battle_trance_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(DrawAction(if info.upgraded { 4 } else { 3 }));
    game.action_queue.push_bot(GainStatusAction {
        status: Status::NoDraw,
        amount: 1,
        target: CreatureRef::player(),
    });
}

pub fn disarm_behavior(game: &mut Game, info: &CardPlayInfo) {
    let amount = if info.upgraded { -3 } else { -2 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Strength,
        amount,
        target: info.target.unwrap(),
    });
}

pub fn rage_behavior(game: &mut Game, info: &CardPlayInfo) {
    let amount = if info.upgraded { 5 } else { 3 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Rage,
        amount,
        target: CreatureRef::player(),
    });
}

pub fn intimidate_behavior(game: &mut Game, info: &CardPlayInfo) {
    let amount = if info.upgraded { 2 } else { 1 };
    game.action_queue.push_bot(GainStatusAllMonstersAction {
        status: Status::Weak,
        amount,
    });
}

pub fn flame_barrier_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 12, 16);
    let amount = if info.upgraded { 6 } else { 4 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::FlameBarrier,
        amount,
        target: CreatureRef::player(),
    });
}

pub fn shockwave_behavior(game: &mut Game, info: &CardPlayInfo) {
    let amount = if info.upgraded { 5 } else { 3 };
    game.action_queue.push_bot(GainStatusAllMonstersAction {
        status: Status::Weak,
        amount,
    });
    game.action_queue.push_bot(GainStatusAllMonstersAction {
        status: Status::Vulnerable,
        amount,
    });
}

pub fn entrench_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(DoubleBlockAction());
}

pub fn power_through_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 15, 20);
    for _ in 0..2 {
        let c = game.new_card(CardClass::Wound);
        game.action_queue.push_bot(PlaceCardInHandAction(c));
    }
}

pub fn burning_pact_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(ChooseCardInHandToExhaustAction());
    game.action_queue
        .push_bot(DrawAction(if info.upgraded { 3 } else { 2 }));
}

pub fn infernal_blade_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(InfernalBladeAction());
}

pub fn second_wind_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(BlockPerNonAttackInHandAction(if info.upgraded {
            7
        } else {
            5
        }));
    game.action_queue.push_bot(ExhaustNonAttackInHandAction());
}

pub fn impervious_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 30, 40);
}

pub fn double_tap_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::DoubleTap,
        amount: if info.upgraded { 2 } else { 1 },
        target: CreatureRef::player(),
    });
}

pub fn offering_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(DamageAction::lose_hp(6, CreatureRef::player()));
    game.action_queue.push_bot(GainEnergyAction(2));
    game.action_queue
        .push_bot(DrawAction(if info.upgraded { 5 } else { 3 }));
}

pub fn exhume_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(ExhumeAction());
}

pub fn limit_break_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(DoubleStrengthAction());
}

pub fn good_instincts_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 6, 9);
}

pub fn finesse_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 2, 4);
    game.action_queue.push_bot(DrawAction(1));
}

pub fn enlightenment_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(EnlightenmentAction {
        for_combat: info.upgraded,
    });
}

pub fn impatience_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(ImpatienceAction(if info.upgraded { 3 } else { 2 }));
}

pub fn jack_of_all_trades_behavior(game: &mut Game, info: &CardPlayInfo) {
    let count = if info.upgraded { 2 } else { 1 };
    for _ in 0..count {
        let class = random_colorless(&mut game.rng);
        let c = game.new_card(class);
        game.action_queue.push_bot(PlaceCardInHandAction(c));
    }
}

pub fn forethought_behavior(game: &mut Game, info: &CardPlayInfo) {
    if info.upgraded {
        game.action_queue.push_bot(ChooseForethoughtAnyAction());
    } else {
        game.action_queue.push_bot(ChooseForethoughtOneAction());
    }
}

pub fn bandage_up_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: if info.upgraded { 6 } else { 4 },
    });
}

pub fn blind_behavior(game: &mut Game, info: &CardPlayInfo) {
    if info.upgraded {
        game.action_queue.push_bot(GainStatusAllMonstersAction {
            status: Status::Weak,
            amount: 2,
        });
    } else {
        game.action_queue.push_bot(GainStatusAction {
            status: Status::Weak,
            amount: 2,
            target: info.target.unwrap(),
        });
    }
}

pub fn trip_behavior(game: &mut Game, info: &CardPlayInfo) {
    if info.upgraded {
        game.action_queue.push_bot(GainStatusAllMonstersAction {
            status: Status::Vulnerable,
            amount: 2,
        });
    } else {
        game.action_queue.push_bot(GainStatusAction {
            status: Status::Vulnerable,
            amount: 2,
            target: info.target.unwrap(),
        });
    }
}

pub fn discovery_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(ChooseDiscoveryAction());
}

pub fn dark_shackles_behavior(game: &mut Game, info: &CardPlayInfo) {
    let amount = if info.upgraded { 15 } else { 9 };
    let target = info.target.unwrap();
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Strength,
        amount: -amount,
        target,
    });
    if !game.get_creature(target).has_status(Status::Artifact) {
        game.action_queue.push_bot(GainStatusAction {
            status: Status::GainStrength,
            amount,
            target,
        });
    }
}

pub fn jax_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(DamageAction::lose_hp(3, CreatureRef::player()));
    let amount = if info.upgraded { 3 } else { 2 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Strength,
        amount,
        target: CreatureRef::player(),
    });
}

pub fn panic_button_behavior(game: &mut Game, info: &CardPlayInfo) {
    push_block(game, info, 30, 40);
    game.action_queue.push_bot(GainStatusAction {
        status: Status::NoBlock,
        amount: 1,
        target: CreatureRef::player(),
    });
}

pub fn madness_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(MadnessAction());
}

pub fn purity_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue
        .push_bot(ChooseCardsInHandToExhaustAction(if info.upgraded {
            5
        } else {
            3
        }));
}

pub fn panacea_behavior(game: &mut Game, info: &CardPlayInfo) {
    let amount = if info.upgraded { 2 } else { 1 };
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Artifact,
        amount,
        target: CreatureRef::player(),
    });
}

pub fn bomb_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Bomb3,
        amount: if info.upgraded { 50 } else { 40 },
        target: CreatureRef::player(),
    });
}

pub fn apotheosis_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(UpgradeAllAction());
}

pub fn thinking_ahead_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(DrawAction(2));
    game.action_queue
        .push_bot(ChooseCardInHandToPlaceOnTopOfDrawAction());
}

pub fn secret_technique_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue
        .push_bot(ChooseCardInDrawToPlaceInHandAction(CardType::Skill));
}

pub fn secret_weapon_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue
        .push_bot(ChooseCardInDrawToPlaceInHandAction(CardType::Attack));
}

pub fn metamorphosis_behavior(game: &mut Game, info: &CardPlayInfo) {
    let count = if info.upgraded { 5 } else { 3 };
    for _ in 0..count {
        let class = random_red_attack_in_combat(&mut game.rng);
        let c = game.new_card(class);
        if let CardCost::Cost { base_cost, .. } = &mut c.borrow_mut().cost {
            *base_cost = 0
        }
        game.action_queue.push_bot(ShuffleCardIntoDrawAction(c));
    }
}

pub fn chrysalis_behavior(game: &mut Game, info: &CardPlayInfo) {
    let count = if info.upgraded { 5 } else { 3 };
    for _ in 0..count {
        let class = random_red_skill_in_combat(&mut game.rng);
        let c = game.new_card(class);
        if let CardCost::Cost { base_cost, .. } = &mut c.borrow_mut().cost {
            *base_cost = 0
        }
        game.action_queue.push_bot(ShuffleCardIntoDrawAction(c));
    }
}

pub fn transmutation_behavior(game: &mut Game, info: &CardPlayInfo) {
    for _ in 0..info.energy {
        let class = random_colorless(&mut game.rng);
        let c = if info.upgraded {
            game.new_card_upgraded(class)
        } else {
            game.new_card(class)
        };
        if let CardCost::Cost { base_cost, .. } = &mut c.borrow_mut().cost {
            *base_cost = 0;
        }
        game.action_queue.push_bot(PlaceCardInHandAction(c));
    }
}

pub fn master_of_strategy_behavior(game: &mut Game, info: &CardPlayInfo) {
    let amount = if info.upgraded { 4 } else { 3 };
    game.action_queue.push_bot(DrawAction(amount));
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{
            block::BlockAction, exhaust_card::ExhaustCardAction, gain_status::GainStatusAction,
            remove_status::RemoveStatusAction,
        },
        assert_matches,
        cards::{CardClass, CardColor, CardCost, CardType},
        game::{CreatureRef, GameBuilder, GameStatus, Move},
        monster::Intent,
        monsters::test::{AttackMonster, IntentMonster, NoopMonster},
        status::Status,
    };

    #[test]
    fn test_defend() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Defend)
            .build_combat();
        g.play_card(CardClass::Defend, None);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);
        assert_eq!(g.player.creature.block, 5);
    }

    #[test]
    fn test_upgraded_defend() {
        let mut g = GameBuilder::default()
            .add_card_upgraded(CardClass::Defend)
            .build_combat();
        g.play_card_upgraded(CardClass::Defend, None);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);
        assert_eq!(g.player.creature.block, 8);
    }

    #[test]
    fn test_armaments() {
        {
            let mut g = GameBuilder::default().build_combat();
            g.add_card_to_hand(CardClass::Strike);
            g.add_card_to_hand_upgraded(CardClass::Defend);
            g.add_card_to_hand(CardClass::TwinStrike);
            g.play_card(CardClass::Armaments, None);
            assert_eq!(
                g.valid_moves(),
                vec![
                    Move::Armaments { card_index: 0 },
                    Move::Armaments { card_index: 2 }
                ]
            );

            g.make_move(Move::Armaments { card_index: 0 });
            assert_eq!(g.hand[0].borrow().upgrade_count, 1);
            assert_eq!(g.hand[1].borrow().upgrade_count, 1);
            assert_eq!(g.hand[2].borrow().upgrade_count, 0);
        }

        {
            let mut g = GameBuilder::default().build_combat();
            g.add_card_to_hand(CardClass::Armaments);
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: None,
            });
            assert_eq!(g.valid_moves(), vec![Move::EndTurn]);
        }

        {
            let mut g = GameBuilder::default().build_combat();
            g.add_card_to_hand(CardClass::Armaments);
            g.add_card_to_hand_upgraded(CardClass::Strike);
            g.add_card_to_hand_upgraded(CardClass::Strike);
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: None,
            });
            assert_matches!(g.result(), GameStatus::Combat);
            for c in g.hand {
                assert!(!c.borrow().can_upgrade());
            }
        }

        {
            let mut g = GameBuilder::default().build_combat();
            g.add_card_to_hand(CardClass::Armaments);
            g.add_card_to_hand(CardClass::Strike);
            g.add_card_to_hand_upgraded(CardClass::Strike);
            g.add_card_to_hand_upgraded(CardClass::Strike);
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: None,
            });
            assert_matches!(g.result(), GameStatus::Combat);
            for c in g.hand {
                assert!(!c.borrow().can_upgrade());
            }
        }
    }

    #[test]
    fn test_upgraded_armaments() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand_upgraded(CardClass::Defend);
        g.add_card_to_hand_upgraded(CardClass::SearingBlow);
        g.play_card_upgraded(CardClass::Armaments, None);
        assert!(g.hand[0].borrow().upgrade_count == 1);
        assert!(g.hand[1].borrow().upgrade_count == 1);
        assert!(g.hand[2].borrow().upgrade_count == 2);
    }

    #[test]
    fn test_true_grit() {
        let mut g = GameBuilder::default().build_combat();
        g.energy = 999;
        let mut found_strike = false;
        let mut found_defend = false;
        for _ in 0..100 {
            g.hand.clear();
            g.exhaust_pile.clear();
            g.add_card_to_hand(CardClass::Strike);
            g.add_card_to_hand(CardClass::Defend);
            g.play_card(CardClass::TrueGrit, None);
            assert_eq!(g.exhaust_pile.len(), 1);
            if g.hand[0].borrow().class == CardClass::Strike {
                found_strike = true;
            } else {
                found_defend = true;
            }
            if found_strike && found_defend {
                break;
            }
        }
        assert!(found_strike);
        assert!(found_defend);
    }

    #[test]
    fn test_havoc() {
        let mut g = GameBuilder::default().build_combat();
        let hp = g.monsters[0].creature.cur_hp;

        g.play_card_upgraded(CardClass::Havoc, None);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 0);

        g.discard_pile.clear();
        g.add_card_to_draw_pile(CardClass::Strike);
        g.play_card(CardClass::Havoc, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6);
        assert_eq!(g.energy, 2);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 1);

        g.discard_pile.clear();
        g.exhaust_pile.clear();
        g.add_card_to_draw_pile(CardClass::TwinStrike);
        g.play_card_upgraded(CardClass::Havoc, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6 - 10);
        assert_eq!(g.energy, 2);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 1);

        g.discard_pile.clear();
        g.exhaust_pile.clear();
        g.add_card_to_draw_pile(CardClass::Whirlwind);
        g.play_card_upgraded(CardClass::Havoc, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6 - 10 - 10);
        assert_eq!(g.energy, 2);
    }

    #[test]
    fn test_warcry() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::Warcry, None);
        assert_matches!(g.result(), GameStatus::Combat);

        g.add_card_to_draw_pile(CardClass::Strike);
        g.play_card(CardClass::Warcry, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.draw_pile.len(), 1);
        assert_eq!(g.hand.len(), 0);

        g.run_action(GainStatusAction {
            status: Status::NoDraw,
            amount: 1,
            target: CreatureRef::player(),
        });
        g.add_card_to_hand(CardClass::Defend);
        g.play_card(CardClass::Warcry, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.draw_pile.len(), 2);
        assert_eq!(g.draw_pile[0].borrow().class, CardClass::Strike);
        assert_eq!(g.draw_pile[1].borrow().class, CardClass::Defend);
        assert_eq!(g.hand.len(), 0);

        g.run_action(RemoveStatusAction {
            status: Status::NoDraw,
            target: CreatureRef::player(),
        });
        g.play_card_upgraded(CardClass::Warcry, None);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::PlaceCardInHandOnTopOfDraw { card_index: 0 },
                Move::PlaceCardInHandOnTopOfDraw { card_index: 1 },
            ]
        );
        g.make_move(Move::PlaceCardInHandOnTopOfDraw { card_index: 0 });
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.draw_pile.len(), 1);
        assert_eq!(g.draw_pile[0].borrow().class, CardClass::Defend);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Strike);
    }

    #[test]
    fn test_havoc_multiple_monsters() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;

        g.add_card_to_draw_pile(CardClass::Strike);
        g.play_card(CardClass::Havoc, None);
        assert!(g.monsters[0].creature.cur_hp == hp || g.monsters[1].creature.cur_hp == hp);
        assert!(g.monsters[0].creature.cur_hp == hp - 6 || g.monsters[1].creature.cur_hp == hp - 6);
    }

    #[test]
    fn test_bloodletting() {
        let mut g = GameBuilder::default().build_combat();
        let hp = g.player.creature.cur_hp;
        g.run_action(BlockAction::player_flat_amount(5));
        g.play_card(CardClass::Bloodletting, None);
        assert_eq!(g.energy, 5);
        assert_eq!(g.player.creature.cur_hp, hp - 3);
    }

    #[test]
    fn test_sentinel() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Sentinel, 2)
            .build_combat();
        assert_eq!(g.energy, 3);
        let c = g.hand.pop().unwrap();
        g.run_action(ExhaustCardAction(c));
        assert_eq!(g.energy, 5);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.player.creature.block, 5);
    }

    #[test]
    fn test_sentinel_upgraded() {
        let mut g = GameBuilder::default()
            .add_cards_upgraded(CardClass::Sentinel, 2)
            .build_combat();
        assert_eq!(g.energy, 3);
        let c = g.hand.pop().unwrap();
        g.run_action(ExhaustCardAction(c));
        assert_eq!(g.energy, 6);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.player.creature.block, 8);
    }

    #[test]
    fn test_spot_weakness() {
        let mut g = GameBuilder::default()
            .add_monster(IntentMonster::new(Intent::Buff))
            .add_monster(IntentMonster::new(Intent::Attack(2, 2)))
            .build_combat();
        g.play_card(CardClass::SpotWeakness, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.get_status(Status::Strength), None);
        g.play_card(CardClass::SpotWeakness, Some(CreatureRef::monster(1)));
        assert_eq!(g.player.creature.get_status(Status::Strength), Some(3));
    }

    #[test]
    fn test_dual_wield() {
        let mut g = GameBuilder::default().build_combat();
        g.energy = 10;
        g.play_card(CardClass::DualWield, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 0);

        g.add_card_to_hand(CardClass::Defend);
        g.play_card(CardClass::DualWield, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 1);

        g.hand.clear();
        g.add_card_to_hand(CardClass::Strike);
        g.play_card(CardClass::DualWield, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 2);
        assert_eq!(g.hand[0].borrow().class, CardClass::Strike);
        assert_eq!(g.hand[1].borrow().class, CardClass::Strike);

        g.hand.clear();
        g.add_card_to_hand(CardClass::Inflame);
        g.play_card_upgraded(CardClass::DualWield, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 3);
        assert_eq!(g.hand[0].borrow().class, CardClass::Inflame);
        assert_eq!(g.hand[1].borrow().class, CardClass::Inflame);
        assert_eq!(g.hand[2].borrow().class, CardClass::Inflame);

        g.hand.clear();
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::Inflame);
        g.add_card_to_hand(CardClass::Defend);
        g.play_card(CardClass::DualWield, None);
        assert_matches!(g.result(), GameStatus::DualWield);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::DualWield { card_index: 0 },
                Move::DualWield { card_index: 1 },
            ]
        );
        g.make_move(Move::DualWield { card_index: 0 });
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 4);
        assert_eq!(g.hand[0].borrow().class, CardClass::Inflame);
        assert_eq!(g.hand[1].borrow().class, CardClass::Defend);
        assert_eq!(g.hand[2].borrow().class, CardClass::Strike);
        assert_eq!(g.hand[3].borrow().class, CardClass::Strike);

        g.hand.clear();
        g.add_card_to_hand(CardClass::RitualDagger);
        let id = g.hand[0].borrow().id;
        g.play_card(CardClass::DualWield, None);
        assert_eq!(g.hand[0].borrow().class, CardClass::RitualDagger);
        assert_eq!(g.hand[0].borrow().id, id);
        assert_eq!(g.hand[1].borrow().class, CardClass::RitualDagger);
        assert_ne!(g.hand[1].borrow().id, id);

        g.hand.clear();
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::RitualDagger);
        let id = g.hand[1].borrow().id;
        g.play_card(CardClass::DualWield, None);
        g.make_move(Move::DualWield { card_index: 1 });
        assert_eq!(g.hand[1].borrow().class, CardClass::RitualDagger);
        assert_ne!(g.hand[1].borrow().id, id);
        assert_eq!(g.hand[2].borrow().class, CardClass::RitualDagger);
        assert_ne!(g.hand[2].borrow().id, id);

        g.hand.clear();
        g.discard_pile.clear();
        for _ in 0..9 {
            g.add_card_to_hand(CardClass::Defend);
        }
        g.add_card_to_hand(CardClass::Strike);
        g.play_card_upgraded(CardClass::DualWield, None);
        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.discard_pile[0].borrow().class, CardClass::Strike);
        assert_eq!(g.discard_pile[1].borrow().class, CardClass::Strike);
        assert_eq!(g.discard_pile[2].borrow().class, CardClass::DualWield);
    }

    #[test]
    fn test_battle_trance() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 20)
            .build_combat();
        assert_eq!(g.hand.len(), 5);
        g.play_card(CardClass::BattleTrance, None);
        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.player.creature.get_status(Status::NoDraw), Some(1));
        g.play_card(CardClass::BattleTrance, None);
        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.player.creature.get_status(Status::NoDraw), Some(1));
        g.make_move(Move::EndTurn);
        assert_eq!(g.hand.len(), 5);
        assert_eq!(g.player.creature.get_status(Status::NoDraw), None);
    }

    #[test]
    fn test_entrench() {
        let mut g = GameBuilder::default().build_combat();
        g.energy = 99;

        g.play_card(CardClass::Entrench, None);
        assert_eq!(g.player.creature.block, 0);

        g.player.creature.block = 10;
        g.play_card(CardClass::Entrench, None);
        assert_eq!(g.player.creature.block, 20);

        g.player.creature.set_status(Status::Dexterity, 3);
        g.play_card(CardClass::Entrench, None);
        assert_eq!(g.player.creature.block, 40);
    }

    #[test]
    fn test_second_wind() {
        let mut g = GameBuilder::default().build_combat();
        g.energy = 99;

        g.play_card(CardClass::SecondWind, None);
        assert_eq!(g.player.creature.block, 0);

        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::Wound);
        g.add_card_to_hand(CardClass::AscendersBane);
        g.play_card(CardClass::SecondWind, None);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Strike);
        assert_eq!(g.exhaust_pile.len(), 3);
        assert_eq!(g.exhaust_pile[0].borrow().class, CardClass::Defend);
        assert_eq!(g.exhaust_pile[1].borrow().class, CardClass::Wound);
        assert_eq!(g.exhaust_pile[2].borrow().class, CardClass::AscendersBane);
        assert_eq!(g.player.creature.block, 15);
    }

    #[test]
    fn test_power_through() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::PowerThrough, None);
        assert_eq!(g.hand.len(), 2);
        assert_eq!(g.hand[0].borrow().class, CardClass::Wound);
        assert_eq!(g.hand[1].borrow().class, CardClass::Wound);
    }

    #[test]
    fn test_burning_pact() {
        let mut g = GameBuilder::default().build_combat();

        g.play_card(CardClass::BurningPact, None);
        assert_matches!(g.result(), GameStatus::Combat);

        g.discard_pile.clear();
        g.add_card_to_draw_pile(CardClass::Strike);
        g.add_card_to_hand(CardClass::Strike);
        g.play_card(CardClass::BurningPact, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.exhaust_pile.len(), 1);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.discard_pile.len(), 1);

        g.hand.clear();
        g.exhaust_pile.clear();
        g.discard_pile.clear();
        g.draw_pile.clear();
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_draw_pile(CardClass::TwinStrike);
        g.play_card(CardClass::BurningPact, None);
        assert_eq!(
            g.cur_card.clone().unwrap().borrow().class,
            CardClass::BurningPact
        );
        assert_matches!(g.result(), GameStatus::ExhaustOneCardInHand);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ExhaustOneCardInHand { card_index: 0 },
                Move::ExhaustOneCardInHand { card_index: 1 },
            ]
        );
        g.make_move(Move::ExhaustOneCardInHand { card_index: 1 });
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.exhaust_pile.len(), 1);
        assert_eq!(g.exhaust_pile[0].borrow().class, CardClass::Defend);
        assert_eq!(g.hand.len(), 2);
    }

    #[test]
    fn test_infernal_blade() {
        let mut g = GameBuilder::default().build_combat();
        for _ in 0..100 {
            g.hand.clear();
            g.play_card_upgraded(CardClass::InfernalBlade, None);
            let c = g.hand[0].borrow();
            if let CardCost::Cost { temporary_cost, .. } = c.cost {
                assert_eq!(temporary_cost, Some(0))
            }
            assert_eq!(c.class.ty(), CardType::Attack);
            assert_ne!(c.class, CardClass::Reaper);
            assert_ne!(c.class, CardClass::Feed);
        }
    }

    #[test]
    fn test_impervious() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::Impervious, None);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.exhaust_pile.len(), 1);
        assert_eq!(g.draw_pile.len(), 0);
        assert_eq!(g.player.creature.block, 30);
    }

    #[test]
    fn test_exhume() {
        let mut g = GameBuilder::default().build_combat();

        g.play_card_upgraded(CardClass::Exhume, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 0);
        assert_eq!(g.exhaust_pile.len(), 1);

        g.hand.clear();
        g.exhaust_pile.clear();
        g.add_card_to_exhaust_pile(CardClass::Strike);
        g.play_card_upgraded(CardClass::Exhume, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Strike);
        assert_eq!(g.exhaust_pile.len(), 1);

        g.hand.clear();
        g.exhaust_pile.clear();
        g.add_card_to_exhaust_pile(CardClass::Exhume);
        g.play_card_upgraded(CardClass::Exhume, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 0);
        assert_eq!(g.exhaust_pile.len(), 2);

        g.hand.clear();
        g.exhaust_pile.clear();
        g.add_card_to_exhaust_pile(CardClass::Strike);
        g.add_card_to_exhaust_pile(CardClass::Exhume);
        g.play_card_upgraded(CardClass::Exhume, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Strike);
        assert_eq!(g.exhaust_pile.len(), 2);

        g.hand.clear();
        g.exhaust_pile.clear();
        g.add_card_to_exhaust_pile(CardClass::Strike);
        g.add_card_to_exhaust_pile(CardClass::Exhume);
        g.add_card_to_exhaust_pile(CardClass::Defend);
        g.play_card_upgraded(CardClass::Exhume, None);
        assert_matches!(g.result(), GameStatus::Exhume);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::Exhume { card_index: 0 },
                Move::Exhume { card_index: 2 },
            ]
        );
        g.make_move(Move::Exhume { card_index: 2 });
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Defend);
        assert_eq!(g.exhaust_pile.len(), 3);
    }

    #[test]
    fn test_limit_break() {
        {
            let mut g = GameBuilder::default()
                .add_player_status(Status::Strength, 3)
                .build_combat();
            g.play_card(CardClass::LimitBreak, None);
            assert_eq!(g.discard_pile.len(), 0);
            assert_eq!(g.exhaust_pile.len(), 1);
            assert_eq!(g.player.creature.get_status(Status::Strength), Some(6));
            g.play_card_upgraded(CardClass::LimitBreak, None);
            assert_eq!(g.discard_pile.len(), 1);
            assert_eq!(g.exhaust_pile.len(), 1);
            assert_eq!(g.player.creature.get_status(Status::Strength), Some(12));
        }
        {
            let mut g = GameBuilder::default()
                .add_player_status(Status::Strength, -3)
                .add_card(CardClass::LimitBreak)
                .build_combat();
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: None,
            });
            assert_eq!(g.player.creature.get_status(Status::Strength), Some(-6));
        }
    }

    #[test]
    fn test_enlightenment() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Enlightenment);
        g.add_card_to_hand(CardClass::SwiftStrike);
        g.add_card_to_hand(CardClass::Bash);
        assert_eq!(g.energy, 3);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.energy, 3);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.energy, 2);
        g.hand.push(g.discard_pile.pop().unwrap());
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.energy, 0);
    }

    #[test]
    fn test_enlightenment_exhaust() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Enlightenment);
        g.add_card_to_hand(CardClass::Bash);
        assert_eq!(g.energy, 3);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        let b = g.hand.pop().unwrap();
        g.run_action(ExhaustCardAction(b));
        g.hand.push(g.exhaust_pile.pop().unwrap());
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.energy, 1);
    }

    #[test]
    fn test_enlightenment_end_turn() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Enlightenment);
        g.add_card_to_hand(CardClass::Bash);
        assert_eq!(g.energy, 3);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        g.discard_pile.pop();
        g.make_move(Move::EndTurn);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.energy, 1);
    }

    #[test]
    fn test_enlightenment_upgraded() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::SwiftStrike);
        g.add_card_to_hand(CardClass::Bash);
        assert_eq!(g.energy, 3);
        g.play_card_upgraded(CardClass::Enlightenment, None);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.energy, 3);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.energy, 2);
        g.hand.push(g.discard_pile.pop().unwrap());
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.energy, 1);
    }

    #[test]
    fn test_enlightenment_upgraded_end_turn() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Bash);
        assert_eq!(g.energy, 3);
        g.play_card_upgraded(CardClass::Enlightenment, None);
        g.discard_pile.pop();
        g.make_move(Move::EndTurn);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.energy, 2);
    }

    #[test]
    fn test_impatience() {
        let mut g = GameBuilder::default().build_combat();
        for _ in 0..50 {
            g.add_card_to_draw_pile(CardClass::Strike);
        }

        g.play_card(CardClass::Impatience, None);
        assert_eq!(g.hand.len(), 2);

        g.play_card(CardClass::Impatience, None);
        assert_eq!(g.hand.len(), 2);

        g.hand.clear();
        g.add_card_to_hand(CardClass::Slimed);
        g.add_card_to_hand(CardClass::AscendersBane);
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::DemonForm);
        g.play_card_upgraded(CardClass::Impatience, None);
        assert_eq!(g.hand.len(), 7);

        g.play_card_upgraded(CardClass::Impatience, None);
        assert_eq!(g.hand.len(), 7);
    }

    #[test]
    fn test_jack_of_all_trades() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::JackOfAllTrades, None);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class.color(), CardColor::Colorless);
        g.play_card_upgraded(CardClass::JackOfAllTrades, None);
        assert_eq!(g.hand.len(), 3);
        assert_eq!(g.hand[0].borrow().class.color(), CardColor::Colorless);
        assert_eq!(g.hand[1].borrow().class.color(), CardColor::Colorless);
        assert_eq!(g.hand[2].borrow().class.color(), CardColor::Colorless);
    }

    #[test]
    fn test_bomb() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::with_hp(1000))
            .add_monster(AttackMonster::with_hp(2, 10))
            .build_combat();
        g.energy = 999;

        let hp = g.monsters[0].creature.cur_hp;

        g.play_card_upgraded(CardClass::Bomb, None);
        assert_eq!(g.player.creature.get_status(Status::Bomb3), Some(50));
        assert_eq!(g.player.creature.get_status(Status::Bomb2), None);
        assert_eq!(g.player.creature.get_status(Status::Bomb1), None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);

        g.play_card(CardClass::Bomb, None);
        assert_eq!(g.player.creature.get_status(Status::Bomb3), Some(90));
        assert_eq!(g.player.creature.get_status(Status::Bomb2), None);
        assert_eq!(g.player.creature.get_status(Status::Bomb1), None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);

        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.get_status(Status::Bomb3), None);
        assert_eq!(g.player.creature.get_status(Status::Bomb2), Some(90));
        assert_eq!(g.player.creature.get_status(Status::Bomb1), None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);

        g.play_card(CardClass::Bomb, None);
        assert_eq!(g.player.creature.get_status(Status::Bomb3), Some(40));
        assert_eq!(g.player.creature.get_status(Status::Bomb2), Some(90));
        assert_eq!(g.player.creature.get_status(Status::Bomb1), None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);

        let player_hp = g.player.creature.cur_hp;
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.get_status(Status::Bomb3), None);
        assert_eq!(g.player.creature.get_status(Status::Bomb2), Some(40));
        assert_eq!(g.player.creature.get_status(Status::Bomb1), Some(90));
        assert_eq!(g.monsters[0].creature.cur_hp, hp);
        assert_eq!(g.player.creature.cur_hp, player_hp - 2);

        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.get_status(Status::Bomb3), None);
        assert_eq!(g.player.creature.get_status(Status::Bomb2), None);
        assert_eq!(g.player.creature.get_status(Status::Bomb1), Some(40));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 90);
        assert_eq!(g.player.creature.cur_hp, player_hp - 2);
    }

    #[test]
    fn test_apotheosis() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand_upgraded(CardClass::Strike);
        g.add_card_to_hand_upgraded(CardClass::SearingBlow);
        g.add_card_to_exhaust_pile(CardClass::Strike);
        g.add_card_to_discard_pile(CardClass::Strike);
        g.add_card_to_discard_pile(CardClass::AscendersBane);
        g.add_card_to_draw_pile(CardClass::Strike);
        g.add_card_to_draw_pile(CardClass::Wound);
        g.play_card(CardClass::Apotheosis, None);
        assert_eq!(g.hand[0].borrow().upgrade_count, 1);
        assert_eq!(g.hand[1].borrow().upgrade_count, 1);
        assert_eq!(g.hand[2].borrow().upgrade_count, 2);
        assert_eq!(g.discard_pile[0].borrow().upgrade_count, 1);
        assert_eq!(g.discard_pile[1].borrow().upgrade_count, 0);
        assert_eq!(g.draw_pile[0].borrow().upgrade_count, 1);
        assert_eq!(g.draw_pile[1].borrow().upgrade_count, 0);
        assert_eq!(g.exhaust_pile[0].borrow().upgrade_count, 1);
    }

    #[test]
    fn test_madness() {
        let mut g = GameBuilder::default().build_combat();

        g.add_card_to_hand(CardClass::Bloodletting);
        g.play_card_upgraded(CardClass::Madness, None);
        assert_eq!(
            g.hand[0].borrow().cost,
            CardCost::Cost {
                base_cost: 0,
                temporary_cost: None,
                free_to_play_once: false,
            }
        );

        g.hand.clear();
        g.add_card_to_hand(CardClass::Strike);
        g.play_card_upgraded(CardClass::Madness, None);
        assert_eq!(
            g.hand[0].borrow().cost,
            CardCost::Cost {
                base_cost: 0,
                temporary_cost: None,
                free_to_play_once: false,
            }
        );

        g.hand.clear();
        let c = g.new_card(CardClass::Strike);
        c.borrow_mut().set_cost(1, Some(0));
        g.hand.push(c);
        g.add_card_to_hand(CardClass::Strike);
        g.play_card_upgraded(CardClass::Madness, None);
        assert_eq!(
            g.hand[0].borrow().cost,
            CardCost::Cost {
                base_cost: 1,
                temporary_cost: Some(0),
                free_to_play_once: false,
            }
        );
        assert_eq!(
            g.hand[1].borrow().cost,
            CardCost::Cost {
                base_cost: 0,
                temporary_cost: None,
                free_to_play_once: false,
            }
        );

        let mut found_0 = false;
        let mut found_1 = false;
        for _ in 0..100 {
            g.hand.clear();
            g.add_card_to_hand(CardClass::Strike);
            g.add_card_to_hand(CardClass::Bash);
            g.play_card_upgraded(CardClass::Madness, None);
            found_0 |= g.hand[0].borrow().get_base_cost() == 0;
            found_1 |= g.hand[1].borrow().get_base_cost() == 0;
        }
        assert!(found_0);
        assert!(found_1);
    }

    #[test]
    fn test_purity() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::Purity, None);
        assert_matches!(g.result(), GameStatus::Combat);

        for _ in 0..4 {
            g.add_card_to_hand(CardClass::Strike);
        }
        for _ in 0..4 {
            g.add_card_to_hand(CardClass::Defend);
        }
        g.play_card(CardClass::Purity, None);
        assert_matches!(
            g.result(),
            GameStatus::ExhaustCardsInHand {
                num_cards_remaining: 3
            }
        );
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ExhaustCardsInHandEnd,
                Move::ExhaustCardsInHand { card_index: 0 },
                Move::ExhaustCardsInHand { card_index: 1 },
                Move::ExhaustCardsInHand { card_index: 2 },
                Move::ExhaustCardsInHand { card_index: 3 },
                Move::ExhaustCardsInHand { card_index: 4 },
                Move::ExhaustCardsInHand { card_index: 5 },
                Move::ExhaustCardsInHand { card_index: 6 },
                Move::ExhaustCardsInHand { card_index: 7 },
            ]
        );
        g.make_move(Move::ExhaustCardsInHandEnd);
        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.exhaust_pile.len(), 1 + 1);

        g.play_card(CardClass::Purity, None);
        assert_matches!(
            g.result(),
            GameStatus::ExhaustCardsInHand {
                num_cards_remaining: 3
            }
        );
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ExhaustCardsInHandEnd,
                Move::ExhaustCardsInHand { card_index: 0 },
                Move::ExhaustCardsInHand { card_index: 1 },
                Move::ExhaustCardsInHand { card_index: 2 },
                Move::ExhaustCardsInHand { card_index: 3 },
                Move::ExhaustCardsInHand { card_index: 4 },
                Move::ExhaustCardsInHand { card_index: 5 },
                Move::ExhaustCardsInHand { card_index: 6 },
                Move::ExhaustCardsInHand { card_index: 7 },
            ]
        );
        g.make_move(Move::ExhaustCardsInHand { card_index: 3 });
        assert_matches!(
            g.result(),
            GameStatus::ExhaustCardsInHand {
                num_cards_remaining: 2
            }
        );
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ExhaustCardsInHandEnd,
                Move::ExhaustCardsInHand { card_index: 0 },
                Move::ExhaustCardsInHand { card_index: 1 },
                Move::ExhaustCardsInHand { card_index: 2 },
                Move::ExhaustCardsInHand { card_index: 3 },
                Move::ExhaustCardsInHand { card_index: 4 },
                Move::ExhaustCardsInHand { card_index: 5 },
                Move::ExhaustCardsInHand { card_index: 6 },
            ]
        );
        g.make_move(Move::ExhaustCardsInHand { card_index: 6 });
        assert_matches!(
            g.result(),
            GameStatus::ExhaustCardsInHand {
                num_cards_remaining: 1
            }
        );
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ExhaustCardsInHandEnd,
                Move::ExhaustCardsInHand { card_index: 0 },
                Move::ExhaustCardsInHand { card_index: 1 },
                Move::ExhaustCardsInHand { card_index: 2 },
                Move::ExhaustCardsInHand { card_index: 3 },
                Move::ExhaustCardsInHand { card_index: 4 },
                Move::ExhaustCardsInHand { card_index: 5 },
            ]
        );
        g.make_move(Move::ExhaustCardsInHand { card_index: 5 });
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 5);
        assert_eq!(g.exhaust_pile.len(), 1 + 1 + 1 + 3);
        assert_eq!(g.hand[0].borrow().class, CardClass::Strike);
        assert_eq!(g.hand[1].borrow().class, CardClass::Strike);
        assert_eq!(g.hand[2].borrow().class, CardClass::Strike);
        assert_eq!(g.hand[3].borrow().class, CardClass::Defend);
        assert_eq!(g.hand[4].borrow().class, CardClass::Defend);

        g.hand.pop();
        g.play_card_upgraded(CardClass::Purity, None);
        g.make_move(Move::ExhaustCardsInHand { card_index: 0 });
        g.make_move(Move::ExhaustCardsInHand { card_index: 0 });
        g.make_move(Move::ExhaustCardsInHand { card_index: 0 });
        g.make_move(Move::ExhaustCardsInHand { card_index: 0 });
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 0);
        assert_eq!(g.discard_pile.len(), 0);
    }

    #[test]
    fn test_secret_weapon() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::SecretWeapon);
        assert_eq!(g.valid_moves(), vec![Move::EndTurn]);

        g.hand.clear();
        g.add_card_to_draw_pile(CardClass::Strike);
        g.add_card_to_draw_pile(CardClass::Defend);
        g.play_card(CardClass::SecretWeapon, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.draw_pile.len(), 1);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Strike);

        g.draw_pile.clear();
        g.hand.clear();
        g.add_card_to_draw_pile(CardClass::Strike);
        g.add_card_to_draw_pile(CardClass::Defend);
        g.add_card_to_draw_pile(CardClass::TwinStrike);
        g.play_card(CardClass::SecretWeapon, None);
        assert_matches!(g.result(), GameStatus::FetchCardFromDraw);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::FetchCardFromDraw { card_index: 0 },
                Move::FetchCardFromDraw { card_index: 2 },
            ]
        );
        g.make_move(Move::FetchCardFromDraw { card_index: 2 });
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.draw_pile.len(), 2);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::TwinStrike);

        g.draw_pile.clear();
        g.hand.clear();
        g.add_card_to_draw_pile(CardClass::Strike);
        for _ in 0..10 {
            g.add_card_to_hand(CardClass::Defend);
        }
        g.play_card(CardClass::SecretWeapon, None);
        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.discard_pile[0].borrow().class, CardClass::Strike);
    }

    #[test]
    fn test_secret_technique() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::SecretTechnique);
        assert_eq!(g.valid_moves(), vec![Move::EndTurn]);

        g.hand.clear();
        g.add_card_to_draw_pile(CardClass::Defend);
        g.add_card_to_draw_pile(CardClass::Strike);
        g.play_card(CardClass::SecretTechnique, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.draw_pile.len(), 1);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Defend);

        g.draw_pile.clear();
        g.hand.clear();
        g.add_card_to_draw_pile(CardClass::Defend);
        g.add_card_to_draw_pile(CardClass::Strike);
        g.add_card_to_draw_pile(CardClass::FlameBarrier);
        g.play_card(CardClass::SecretTechnique, None);
        assert_matches!(g.result(), GameStatus::FetchCardFromDraw);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::FetchCardFromDraw { card_index: 0 },
                Move::FetchCardFromDraw { card_index: 2 },
            ]
        );
        g.make_move(Move::FetchCardFromDraw { card_index: 2 });
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.draw_pile.len(), 2);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::FlameBarrier);

        g.draw_pile.clear();
        g.hand.clear();
        g.add_card_to_draw_pile(CardClass::Defend);
        for _ in 0..10 {
            g.add_card_to_hand(CardClass::Strike);
        }
        g.play_card(CardClass::SecretTechnique, None);
        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.discard_pile[0].borrow().class, CardClass::Defend);
    }

    #[test]
    fn test_metamorphosis() {
        let mut g = GameBuilder::default().build_combat();
        for _ in 0..100 {
            g.energy = 3;
            g.draw_pile.clear();
            g.play_card(CardClass::Metamorphosis, None);
            assert_eq!(g.draw_pile.len(), 3);

            for c in &g.draw_pile {
                assert_eq!(c.borrow().class.ty(), CardType::Attack);
                assert_eq!(c.borrow().class.color(), CardColor::Red);
                assert_ne!(c.borrow().class, CardClass::Reaper);
                assert_ne!(c.borrow().class, CardClass::Feed);
                if c.borrow().class != CardClass::Whirlwind {
                    assert_matches!(
                        c.borrow().cost,
                        CardCost::Cost {
                            base_cost: 0,
                            temporary_cost: None,
                            free_to_play_once: false,
                        }
                    );
                }
            }
        }
    }

    #[test]
    fn test_chrysalis() {
        let mut g = GameBuilder::default().build_combat();
        for _ in 0..100 {
            g.energy = 3;
            g.draw_pile.clear();
            g.play_card(CardClass::Chrysalis, None);
            assert_eq!(g.draw_pile.len(), 3);

            for c in &g.draw_pile {
                assert_eq!(c.borrow().class.ty(), CardType::Skill);
                assert_eq!(c.borrow().class.color(), CardColor::Red);
                assert_ne!(c.borrow().class, CardClass::Reaper);
                assert_ne!(c.borrow().class, CardClass::Feed);
                assert_matches!(
                    c.borrow().cost,
                    CardCost::Cost {
                        base_cost: 0,
                        temporary_cost: None,
                        free_to_play_once: false,
                    }
                );
            }
        }
    }

    #[test]
    fn test_transmutation() {
        let mut g = GameBuilder::default().build_combat();

        g.energy = 0;
        g.play_card(CardClass::Transmutation, None);
        assert_eq!(g.hand.len(), 0);

        for _ in 0..50 {
            g.hand.clear();
            g.energy = 2;
            g.play_card(CardClass::Transmutation, None);
            assert_eq!(g.energy, 0);
            assert_eq!(g.hand.len(), 2);
            for c in &g.hand {
                assert_eq!(c.borrow().class.color(), CardColor::Colorless);
                assert_eq!(c.borrow().upgrade_count, 0);
                if let CardCost::Cost {
                    base_cost,
                    temporary_cost,
                    free_to_play_once,
                } = c.borrow().cost
                {
                    assert_eq!(base_cost, 0);
                    assert_eq!(temporary_cost, None);
                    assert!(!free_to_play_once);
                }
            }

            g.hand.clear();
            g.energy = 2;
            g.play_card_upgraded(CardClass::Transmutation, None);
            assert_eq!(g.energy, 0);
            assert_eq!(g.hand.len(), 2);
            for c in &g.hand {
                assert_eq!(c.borrow().class.color(), CardColor::Colorless);
                assert_eq!(c.borrow().upgrade_count, 1);
                if let CardCost::Cost {
                    base_cost,
                    temporary_cost,
                    free_to_play_once,
                } = c.borrow().cost
                {
                    assert_eq!(base_cost, 0);
                    assert_eq!(temporary_cost, None);
                    assert!(!free_to_play_once);
                }
            }
        }
    }

    #[test]
    fn test_forethought() {
        let mut g = GameBuilder::default().build_combat();

        g.play_card(CardClass::Forethought, None);
        assert_matches!(g.result(), GameStatus::Combat);

        g.add_card_to_draw_pile(CardClass::Defend);
        g.add_card_to_hand(CardClass::Strike);
        g.play_card(CardClass::Forethought, None);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.draw_pile.len(), 2);
        assert_eq!(g.draw_pile[0].borrow().class, CardClass::Strike);
        match g.draw_pile[0].borrow().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => assert!(free_to_play_once),
            _ => panic!(),
        }
        assert_eq!(g.draw_pile[1].borrow().class, CardClass::Defend);
        match g.draw_pile[1].borrow().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => assert!(!free_to_play_once),
            _ => panic!(),
        }

        g.draw_pile.clear();
        g.hand.clear();
        g.add_card_to_draw_pile(CardClass::Defend);
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::TwinStrike);
        g.play_card(CardClass::Forethought, None);
        assert_matches!(g.result(), GameStatus::ForethoughtOne);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ForethoughtOne { card_index: 0 },
                Move::ForethoughtOne { card_index: 1 }
            ]
        );
        g.make_move(Move::ForethoughtOne { card_index: 0 });
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.draw_pile.len(), 2);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.draw_pile[0].borrow().class, CardClass::Strike);
        match g.draw_pile[0].borrow().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => assert!(free_to_play_once),
            _ => panic!(),
        }
        assert_eq!(g.draw_pile[1].borrow().class, CardClass::Defend);
        match g.draw_pile[1].borrow().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => assert!(!free_to_play_once),
            _ => panic!(),
        }
    }

    #[test]
    fn test_forethought_upgraded() {
        let mut g = GameBuilder::default().build_combat();

        g.play_card_upgraded(CardClass::Forethought, None);
        assert_matches!(g.result(), GameStatus::Combat);

        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::TwinStrike);
        g.add_card_to_draw_pile(CardClass::Defend);
        g.play_card_upgraded(CardClass::Forethought, None);
        assert_matches!(g.result(), GameStatus::ForethoughtAny);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ForethoughtAnyEnd,
                Move::ForethoughtAny { card_index: 0 },
                Move::ForethoughtAny { card_index: 1 }
            ]
        );
        g.make_move(Move::ForethoughtAnyEnd);
        assert_eq!(g.draw_pile.len(), 1);

        g.draw_pile.clear();
        g.hand.clear();
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::TwinStrike);
        g.add_card_to_draw_pile(CardClass::Defend);
        g.play_card_upgraded(CardClass::Forethought, None);
        assert_matches!(g.result(), GameStatus::ForethoughtAny);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ForethoughtAnyEnd,
                Move::ForethoughtAny { card_index: 0 },
                Move::ForethoughtAny { card_index: 1 }
            ]
        );
        g.make_move(Move::ForethoughtAny { card_index: 0 });
        assert_matches!(g.result(), GameStatus::ForethoughtAny);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ForethoughtAnyEnd,
                Move::ForethoughtAny { card_index: 0 }
            ]
        );
        g.make_move(Move::ForethoughtAnyEnd);
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.draw_pile.len(), 2);
        assert_eq!(g.draw_pile[0].borrow().class, CardClass::Strike);
        match g.draw_pile[0].borrow().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => assert!(free_to_play_once),
            _ => panic!(),
        }

        g.draw_pile.clear();
        g.hand.clear();
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::TwinStrike);
        g.add_card_to_draw_pile(CardClass::Defend);
        g.play_card_upgraded(CardClass::Forethought, None);
        assert_matches!(g.result(), GameStatus::ForethoughtAny);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ForethoughtAnyEnd,
                Move::ForethoughtAny { card_index: 0 },
                Move::ForethoughtAny { card_index: 1 }
            ]
        );
        g.make_move(Move::ForethoughtAny { card_index: 1 });
        assert_matches!(g.result(), GameStatus::ForethoughtAny);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::ForethoughtAnyEnd,
                Move::ForethoughtAny { card_index: 0 }
            ]
        );
        g.make_move(Move::ForethoughtAny { card_index: 0 });
        assert_matches!(g.result(), GameStatus::Combat);
        assert_eq!(g.hand.len(), 0);
        assert_eq!(g.draw_pile.len(), 3);
        assert_eq!(g.draw_pile[0].borrow().class, CardClass::TwinStrike);
        match g.draw_pile[0].borrow().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => assert!(free_to_play_once),
            _ => panic!(),
        }
        assert_eq!(g.draw_pile[1].borrow().class, CardClass::Strike);
        match g.draw_pile[1].borrow().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => assert!(free_to_play_once),
            _ => panic!(),
        }
        assert_eq!(g.draw_pile[2].borrow().class, CardClass::Defend);
        match g.draw_pile[2].borrow().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => assert!(!free_to_play_once),
            _ => panic!(),
        }

        g.draw_pile.clear();
        g.hand.clear();
        g.add_card_to_hand(CardClass::Strike);
        g.add_card_to_hand(CardClass::TwinStrike);
        g.add_card_to_draw_pile(CardClass::Defend);
        g.play_card_upgraded(CardClass::Forethought, None);
        g.make_move(Move::ForethoughtAny { card_index: 0 });
        g.make_move(Move::ForethoughtAny { card_index: 0 });
        assert_eq!(g.draw_pile[0].borrow().class, CardClass::Strike);
        assert_eq!(g.draw_pile[1].borrow().class, CardClass::TwinStrike);
    }

    #[test]
    fn test_discovery() {
        for _ in 0..100 {
            let mut g = GameBuilder::default().build_combat();
            g.energy = 1;
            g.play_card(CardClass::Discovery, None);
            for m in g.valid_moves() {
                if let Move::Discovery { card_class } = m {
                    assert_ne!(card_class, CardClass::Reaper);
                } else {
                    panic!();
                }
            }
            assert_eq!(g.valid_moves().len(), 3);
            g.make_move(g.valid_moves()[0]);
            dbg!(g.hand[0].borrow());
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: Some(0),
            });
        }
    }

    #[test]
    fn test_dark_shackles() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::DarkShackles, Some(CreatureRef::monster(0)));
        assert_eq!(
            g.monsters[0].creature.get_status(Status::Strength),
            Some(-9)
        );
        assert_eq!(
            g.monsters[0].creature.get_status(Status::GainStrength),
            Some(9)
        );
        g.make_move(Move::EndTurn);
        assert_eq!(g.monsters[0].creature.get_status(Status::Strength), None);
        assert_eq!(
            g.monsters[0].creature.get_status(Status::GainStrength),
            None
        );

        g.monsters[0].creature.set_status(Status::Artifact, 2);
        g.play_card(CardClass::DarkShackles, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.get_status(Status::Artifact), Some(1));
        assert_eq!(g.monsters[0].creature.get_status(Status::Strength), None);
        assert_eq!(
            g.monsters[0].creature.get_status(Status::GainStrength),
            None
        );
    }

    #[test]
    fn test_panic_button() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::PanicButton, None);
        assert_eq!(g.player.creature.block, 30);
        assert_eq!(g.player.creature.get_status(Status::NoBlock), Some(1));
        g.play_card(CardClass::PanicButton, None);
        assert_eq!(g.player.creature.block, 30);
        assert_eq!(g.player.creature.get_status(Status::NoBlock), Some(2));
    }
}
