use crate::{
    actions::{
        damage::DamageAction, damage_all_monsters::DamageAllMonstersAction, draw::DrawAction,
        gain_status::GainStatusAction, gain_status_all_monsters::GainStatusAllMonstersAction,
        increase_max_hp::IncreaseMaxHPAction, shuffle_card_into_draw::ShuffleCardIntoDrawAction,
    },
    card::CardPlayInfo,
    cards::{CardClass, new_card},
    game::Game,
    status::Status,
};

fn push_damage(
    game: &mut Game,
    info: CardPlayInfo,
    unupgraded_base_damage: i32,
    upgraded_base_damage: i32,
) {
    game.action_queue.push_bot(DamageAction::from_player(
        if info.upgraded {
            upgraded_base_damage
        } else {
            unupgraded_base_damage
        },
        &game.player,
        game.get_creature(info.target.unwrap()),
        info.target.unwrap(),
    ));
}

fn push_aoe_damage(
    game: &mut Game,
    info: CardPlayInfo,
    unupgraded_base_damage: i32,
    upgraded_base_damage: i32,
) {
    game.action_queue
        .push_bot(DamageAllMonstersAction::from_player(if info.upgraded {
            upgraded_base_damage
        } else {
            unupgraded_base_damage
        }));
}

pub fn strike_behavior(game: &mut Game, info: CardPlayInfo) {
    push_damage(game, info, 6, 9);
}

pub fn bash_behavior(game: &mut Game, info: CardPlayInfo) {
    push_damage(game, info, 8, 10);
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Vulnerable,
        amount: if info.upgraded { 3 } else { 2 },
        target: info.target.unwrap(),
    });
}

pub fn pommel_strike_behavior(game: &mut Game, info: CardPlayInfo) {
    push_damage(game, info, 9, 10);
    game.action_queue
        .push_bot(DrawAction(if info.upgraded { 2 } else { 1 }));
}

pub fn twin_strike_behavior(game: &mut Game, info: CardPlayInfo) {
    for _ in 0..2 {
        push_damage(game, info, 5, 7);
    }
}

pub fn clothesline_behavior(game: &mut Game, info: CardPlayInfo) {
    push_damage(game, info, 12, 14);
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Weak,
        amount: if info.upgraded { 3 } else { 2 },
        target: info.target.unwrap(),
    });
}

pub fn cleave_behavior(game: &mut Game, info: CardPlayInfo) {
    push_aoe_damage(game, info, 8, 11);
}

pub fn thunderclap_behavior(game: &mut Game, info: CardPlayInfo) {
    push_aoe_damage(game, info, 4, 7);
    game.action_queue.push_bot(GainStatusAllMonstersAction {
        status: Status::Vulnerable,
        amount: 1,
    });
}

pub fn body_slam_behavior(game: &mut Game, info: CardPlayInfo) {
    let damage = game.player.creature.block;
    push_damage(game, info, damage, damage);
}

pub fn wild_strike_behavior(game: &mut Game, info: CardPlayInfo) {
    push_damage(game, info, 12, 17);
    game.action_queue
        .push_bot(ShuffleCardIntoDrawAction(new_card(CardClass::Wound)));
}

pub fn reckless_charge_behavior(game: &mut Game, info: CardPlayInfo) {
    push_damage(game, info, 7, 10);
    game.action_queue
        .push_bot(ShuffleCardIntoDrawAction(new_card(CardClass::Dazed)));
}

pub fn searing_blow_behavior(game: &mut Game, info: CardPlayInfo) {
    let n = info.upgrade_count;
    game.action_queue.push_bot(DamageAction::from_player(
        n * (n + 7) / 2 + 12,
        &game.player,
        game.get_creature(info.target.unwrap()),
        info.target.unwrap(),
    ));
}

pub fn whirlwind_behavior(game: &mut Game, info: CardPlayInfo) {
    for _ in 0..game.energy {
        push_aoe_damage(game, info, 5, 8);
    }
}

pub fn rampage_behavior(game: &mut Game, info: CardPlayInfo) {
    let damage = if info.upgraded { 8 } else { 5 } * info.times_played + 8;
    push_damage(game, info, damage, damage);
}

pub fn feed_behavior(game: &mut Game, info: CardPlayInfo) {
    if info.upgraded {
        game.action_queue
            .push_bot(DamageAction::from_player_with_on_fatal(
                12,
                &game.player,
                game.get_creature(info.target.unwrap()),
                info.target.unwrap(),
                |queue| queue.push_top(IncreaseMaxHPAction(4)),
            ));
    } else {
        game.action_queue
            .push_bot(DamageAction::from_player_with_on_fatal(
                10,
                &game.player,
                game.get_creature(info.target.unwrap()),
                info.target.unwrap(),
                |queue| queue.push_top(IncreaseMaxHPAction(3)),
            ));
    }
}

pub fn swift_strike_behavior(game: &mut Game, info: CardPlayInfo) {
    push_damage(game, info, 7, 10);
}

pub fn flash_of_steel_behavior(game: &mut Game, info: CardPlayInfo) {
    push_damage(game, info, 3, 6);
    game.action_queue.push_bot(DrawAction(1));
}

pub fn dramatic_entrance_behavior(game: &mut Game, info: CardPlayInfo) {
    push_aoe_damage(game, info, 8, 12);
}

pub fn mind_blast_behavior(game: &mut Game, info: CardPlayInfo) {
    let damage = game.draw_pile.len() as i32;
    push_damage(game, info, damage, damage);
}

pub fn debug_kill_behavior(game: &mut Game, info: CardPlayInfo) {
    push_damage(game, info, 9999, 9999);
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{block::BlockAction, set_hp::SetHPAction},
        cards::{CardClass, new_card, new_card_upgraded},
        game::{CreatureRef, GameBuilder, Move},
        monsters::test::NoopMonster,
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
        assert_eq!(g.monsters[0].creature.statuses.len(), 0);
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
        assert_eq!(g.monsters[0].creature.statuses.len(), 1);
        assert_eq!(
            g.monsters[0].creature.statuses.get(&Status::Vulnerable),
            Some(&2)
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
        assert_eq!(g.draw_pile[0].borrow().class, CardClass::Wound);
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
    fn test_feed() {
        let mut g = GameBuilder::default().build_combat();
        let player_max_hp = g.player.creature.max_hp;
        let player_cur_hp = g.player.creature.cur_hp;
        let monster_hp = g.monsters[0].creature.cur_hp;

        g.play_card(CardClass::Feed, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, monster_hp - 10);
        assert_eq!(g.player.creature.max_hp, player_max_hp);
        assert_eq!(g.player.creature.cur_hp, player_cur_hp);

        g.run_action(SetHPAction {
            target: CreatureRef::monster(0),
            hp: 8,
        });
        g.play_card(CardClass::Feed, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 0);
        assert_eq!(g.player.creature.max_hp, player_max_hp + 3);
        assert_eq!(g.player.creature.cur_hp, player_cur_hp + 3);
    }

    #[test]
    fn test_feed_upgrade() {
        let mut g = GameBuilder::default().build_combat();
        let player_max_hp = g.player.creature.max_hp;
        let player_cur_hp = g.player.creature.cur_hp;
        let monster_hp = g.monsters[0].creature.cur_hp;

        g.play_card_upgraded(CardClass::Feed, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, monster_hp - 12);
        assert_eq!(g.player.creature.max_hp, player_max_hp);
        assert_eq!(g.player.creature.cur_hp, player_cur_hp);

        g.run_action(SetHPAction {
            target: CreatureRef::monster(0),
            hp: 11,
        });
        g.play_card_upgraded(CardClass::Feed, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, 0);
        assert_eq!(g.player.creature.max_hp, player_max_hp + 4);
        assert_eq!(g.player.creature.cur_hp, player_cur_hp + 4);
    }

    #[test]
    fn test_mind_blast() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 25)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.hand.push(new_card(CardClass::MindBlast));
        g.make_move(Move::PlayCard {
            card_index: 5,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 20);
    }

    #[test]
    fn test_debug_kill() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(0)));
        assert!(!g.monsters[0].creature.is_alive());
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
        let c = new_card_upgraded(CardClass::Strike);
        let mut c = c.borrow_mut();
        c.upgrade();
    }
}
