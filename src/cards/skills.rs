use crate::{
    actions::{
        block::BlockAction,
        choose_upgrade_one_card_in_hand::ChooseUpgradeOneCardInHandAction,
        damage::{DamageAction, DamageType},
        double_strength::DoubleStrengthAction,
        draw::DrawAction,
        gain_energy::GainEnergyAction,
        upgrade_all_cards_in_hand::UpgradeAllCardsInHandAction,
    },
    card::CardPlayInfo,
    game::{CreatureRef, Game},
};

fn push_block(
    game: &mut Game,
    info: CardPlayInfo,
    unupgraded_base_damage: i32,
    upgraded_base_damage: i32,
) {
    game.action_queue.push_bot(BlockAction {
        target: CreatureRef::player(),
        amount: if info.upgraded {
            upgraded_base_damage
        } else {
            unupgraded_base_damage
        },
    });
}

pub fn defend_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    push_block(game, info, 5, 8);
}

pub fn armaments_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    push_block(game, info, 5, 5);
    if info.upgraded {
        game.action_queue.push_bot(UpgradeAllCardsInHandAction());
    } else {
        game.action_queue
            .push_bot(ChooseUpgradeOneCardInHandAction());
    }
}

pub fn ghostly_armor_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    push_block(game, info, 10, 13);
}

pub fn bloodletting_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    game.action_queue.push_bot(DamageAction {
        target: CreatureRef::player(),
        amount: 3,
        ty: DamageType::HPLoss,
    });
    game.action_queue.push_bot(GainEnergyAction {
        amount: if info.upgraded { 3 } else { 2 },
    });
}

pub fn impervious_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    push_block(game, info, 30, 40);
}

pub fn limit_break_behavior(game: &mut Game, _: Option<CreatureRef>, _: CardPlayInfo) {
    game.action_queue.push_bot(DoubleStrengthAction());
}

pub fn good_instincts_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    push_block(game, info, 6, 9);
}

pub fn finesse_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    push_block(game, info, 2, 4);
    game.action_queue.push_bot(DrawAction(1));
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::block::BlockAction,
        cards::{CardClass, new_card, new_card_upgraded},
        game::{CreatureRef, GameBuilder, Move},
        status::Status,
    };

    #[test]
    fn test_defend() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Defend)
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
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
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
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
    fn test_bloodletting() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Bloodletting)
            .build_combat();
        let hp = g.player.creature.cur_hp;
        g.run_action(BlockAction {
            target: CreatureRef::player(),
            amount: 5,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 5);
        assert_eq!(g.player.creature.cur_hp, hp - 3);
    }

    #[test]
    fn test_impervious() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Impervious)
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
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
}
