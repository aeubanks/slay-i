use crate::{
    actions::{
        block::BlockAction, choose_upgrade_one_card_in_hand::ChooseUpgradeOneCardInHandAction,
        damage::DamageAction, double_strength::DoubleStrengthAction, draw::DrawAction,
        enlightenment::EnlightenmentAction, gain_energy::GainEnergyAction,
        gain_status::GainStatusAction, madness::MadnessAction, play_top_card::PlayTopCardAction,
        upgrade_all_cards_in_hand::UpgradeAllCardsInHandAction,
    },
    card::CardPlayInfo,
    game::{CreatureRef, Game},
    status::Status,
};

fn push_block(
    game: &mut Game,
    info: CardPlayInfo,
    unupgraded_base_damage: i32,
    upgraded_base_damage: i32,
) {
    game.action_queue
        .push_bot(BlockAction::player_card(if info.upgraded {
            upgraded_base_damage
        } else {
            unupgraded_base_damage
        }));
}

pub fn defend_behavior(game: &mut Game, info: CardPlayInfo) {
    push_block(game, info, 5, 8);
}

pub fn armaments_behavior(game: &mut Game, info: CardPlayInfo) {
    push_block(game, info, 5, 5);
    if info.upgraded {
        game.action_queue.push_bot(UpgradeAllCardsInHandAction());
    } else {
        game.action_queue
            .push_bot(ChooseUpgradeOneCardInHandAction());
    }
}

pub fn havoc_behavior(game: &mut Game, _: CardPlayInfo) {
    game.action_queue.push_bot(PlayTopCardAction {
        force_exhaust: true,
    });
}

pub fn ghostly_armor_behavior(game: &mut Game, info: CardPlayInfo) {
    push_block(game, info, 10, 13);
}

pub fn bloodletting_behavior(game: &mut Game, info: CardPlayInfo) {
    game.action_queue
        .push_bot(DamageAction::lose_hp(3, CreatureRef::player()));
    game.action_queue
        .push_bot(GainEnergyAction(if info.upgraded { 3 } else { 2 }));
}

pub fn sentinel_behavior(game: &mut Game, info: CardPlayInfo) {
    push_block(game, info, 5, 8);
}

pub fn battle_trance_behavior(game: &mut Game, info: CardPlayInfo) {
    game.action_queue
        .push_bot(DrawAction(if info.upgraded { 4 } else { 3 }));
    game.action_queue.push_bot(GainStatusAction {
        status: Status::NoDraw,
        amount: 1,
        target: CreatureRef::player(),
    });
}

pub fn impervious_behavior(game: &mut Game, info: CardPlayInfo) {
    push_block(game, info, 30, 40);
}

pub fn double_tap_behavior(game: &mut Game, info: CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::DoubleTap,
        amount: if info.upgraded { 2 } else { 1 },
        target: CreatureRef::player(),
    });
}

pub fn limit_break_behavior(game: &mut Game, _: CardPlayInfo) {
    game.action_queue.push_bot(DoubleStrengthAction());
}

pub fn good_instincts_behavior(game: &mut Game, info: CardPlayInfo) {
    push_block(game, info, 6, 9);
}

pub fn finesse_behavior(game: &mut Game, info: CardPlayInfo) {
    push_block(game, info, 2, 4);
    game.action_queue.push_bot(DrawAction(1));
}

pub fn enlightenment_behavior(game: &mut Game, info: CardPlayInfo) {
    game.action_queue.push_bot(EnlightenmentAction {
        for_combat: info.upgraded,
    });
}

pub fn madness_behavior(game: &mut Game, _: CardPlayInfo) {
    game.action_queue.push_bot(MadnessAction());
}

pub fn bomb_behavior(game: &mut Game, info: CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Bomb3,
        amount: if info.upgraded { 50 } else { 40 },
        target: CreatureRef::player(),
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{block::BlockAction, exhaust_card::ExhaustCardAction},
        cards::{CardClass, CardCost, new_card, new_card_upgraded},
        game::{GameBuilder, Move},
        monsters::test::{AttackMonster, NoopMonster},
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
            g.hand.push(new_card(CardClass::Armaments));
            g.hand.push(new_card(CardClass::Strike));
            g.hand.push(new_card_upgraded(CardClass::Defend));
            g.hand.push(new_card(CardClass::TwinStrike));
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: None,
            });
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
            g.hand.push(new_card(CardClass::Armaments));
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: None,
            });
            assert_eq!(g.valid_moves(), vec![Move::EndTurn]);
        }
    }

    #[test]
    fn test_upgraded_armaments() {
        let mut g = GameBuilder::default().build_combat();
        g.hand.push(new_card_upgraded(CardClass::Armaments));
        g.hand.push(new_card(CardClass::Strike));
        g.hand.push(new_card_upgraded(CardClass::Defend));
        g.hand.push(new_card_upgraded(CardClass::SearingBlow));
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert!(g.hand[0].borrow().upgrade_count == 1);
        assert!(g.hand[1].borrow().upgrade_count == 1);
        assert!(g.hand[2].borrow().upgrade_count == 2);
    }

    #[test]
    fn test_havoc() {
        let mut g = GameBuilder::default().build_combat();
        let hp = g.monsters[0].creature.cur_hp;

        g.play_card_upgraded(CardClass::Havoc, None);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 0);

        g.discard_pile.clear();
        g.draw_pile.push(new_card(CardClass::Strike));
        g.play_card(CardClass::Havoc, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6);
        assert_eq!(g.energy, 2);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 1);

        g.discard_pile.clear();
        g.exhaust_pile.clear();
        g.draw_pile.push(new_card_upgraded(CardClass::Strike));
        g.play_card_upgraded(CardClass::Havoc, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6 - 9);
        assert_eq!(g.energy, 2);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 1);

        g.discard_pile.clear();
        g.exhaust_pile.clear();
        g.draw_pile.push(new_card(CardClass::Whirlwind));
        g.play_card_upgraded(CardClass::Havoc, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6 - 9 - 10);
        assert_eq!(g.energy, 2);
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
    fn test_battle_trance() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 20)
            .build_combat();
        assert_eq!(g.hand.len(), 5);
        g.play_card(CardClass::BattleTrance, None);
        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.player.creature.statuses.get(&Status::NoDraw), Some(&1));
        g.play_card(CardClass::BattleTrance, None);
        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.player.creature.statuses.get(&Status::NoDraw), Some(&1));
        g.make_move(Move::EndTurn);
        assert_eq!(g.hand.len(), 5);
        assert_eq!(g.player.creature.statuses.get(&Status::NoDraw), None);
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
    fn test_limit_break() {
        {
            let mut g = GameBuilder::default()
                .add_player_status(Status::Strength, 3)
                .build_combat();
            g.hand.push(new_card(CardClass::LimitBreak));
            g.hand.push(new_card_upgraded(CardClass::LimitBreak));
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: None,
            });
            assert_eq!(g.discard_pile.len(), 0);
            assert_eq!(g.exhaust_pile.len(), 1);
            assert_eq!(g.player.creature.statuses.get(&Status::Strength), Some(&6));
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: None,
            });
            assert_eq!(g.discard_pile.len(), 1);
            assert_eq!(g.exhaust_pile.len(), 1);
            assert_eq!(g.player.creature.statuses.get(&Status::Strength), Some(&12));
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
            assert_eq!(g.player.creature.statuses.get(&Status::Strength), Some(&-6));
        }
    }

    #[test]
    fn test_enlightenment() {
        let mut g = GameBuilder::default().build_combat();
        g.hand.push(new_card(CardClass::Enlightenment));
        g.hand.push(new_card(CardClass::SwiftStrike));
        g.hand.push(new_card(CardClass::Bash));
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
        g.hand.push(new_card(CardClass::Enlightenment));
        g.hand.push(new_card(CardClass::Bash));
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
        g.hand.push(new_card(CardClass::Enlightenment));
        g.hand.push(new_card(CardClass::Bash));
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
        g.hand.push(new_card_upgraded(CardClass::Enlightenment));
        g.hand.push(new_card(CardClass::SwiftStrike));
        g.hand.push(new_card(CardClass::Bash));
        assert_eq!(g.energy, 3);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
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
        g.hand.push(new_card_upgraded(CardClass::Enlightenment));
        g.hand.push(new_card(CardClass::Bash));
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
        assert_eq!(g.energy, 2);
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
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb3), Some(&50));
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb2), None);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb1), None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);

        g.play_card(CardClass::Bomb, None);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb3), Some(&90));
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb2), None);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb1), None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);

        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb3), None);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb2), Some(&90));
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb1), None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);

        g.play_card(CardClass::Bomb, None);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb3), Some(&40));
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb2), Some(&90));
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb1), None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);

        let player_hp = g.player.creature.cur_hp;
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb3), None);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb2), Some(&40));
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb1), Some(&90));
        assert_eq!(g.monsters[0].creature.cur_hp, hp);
        assert_eq!(g.player.creature.cur_hp, player_hp - 2);

        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb3), None);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb2), None);
        assert_eq!(g.player.creature.statuses.get(&Status::Bomb1), Some(&40));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 90);
        assert_eq!(g.player.creature.cur_hp, player_hp - 2);
    }

    #[test]
    fn test_madness() {
        let mut g = GameBuilder::default().build_combat();

        g.hand.push(new_card(CardClass::Bloodletting));
        g.play_card_upgraded(CardClass::Madness, None);
        assert_eq!(
            g.hand[0].borrow().cost,
            CardCost::Cost {
                base_cost: 0,
                temporary_cost: None
            }
        );

        g.hand.clear();
        g.hand.push(new_card(CardClass::Strike));
        g.play_card_upgraded(CardClass::Madness, None);
        assert_eq!(
            g.hand[0].borrow().cost,
            CardCost::Cost {
                base_cost: 0,
                temporary_cost: None
            }
        );

        g.hand.clear();
        let c = new_card(CardClass::Strike);
        c.borrow_mut().set_cost(1, Some(0));
        g.hand.push(c);
        g.hand.push(new_card(CardClass::Strike));
        g.play_card_upgraded(CardClass::Madness, None);
        assert_eq!(
            g.hand[0].borrow().cost,
            CardCost::Cost {
                base_cost: 1,
                temporary_cost: Some(0)
            }
        );
        assert_eq!(
            g.hand[1].borrow().cost,
            CardCost::Cost {
                base_cost: 0,
                temporary_cost: None
            }
        );

        let mut found_0 = false;
        let mut found_1 = false;
        for _ in 0..100 {
            g.hand.clear();
            g.hand.push(new_card(CardClass::Strike));
            g.hand.push(new_card(CardClass::Bash));
            g.play_card_upgraded(CardClass::Madness, None);
            found_0 |= g.hand[0].borrow().get_base_cost() == 0;
            found_1 |= g.hand[1].borrow().get_base_cost() == 0;
        }
        assert!(found_0);
        assert!(found_1);
    }
}
