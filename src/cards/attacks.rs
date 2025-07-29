use crate::{
    actions::{damage::DamageAction, draw::DrawAction, gain_status::GainStatusAction},
    card::CardPlayInfo,
    game::{CreatureRef, Game},
    status::Status,
};

fn push_damage(
    game: &mut Game,
    target: Option<CreatureRef>,
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
        game.get_creature(target.unwrap()),
        target.unwrap(),
    ));
}

fn push_aoe_damage(
    game: &mut Game,
    info: CardPlayInfo,
    unupgraded_base_damage: i32,
    upgraded_base_damage: i32,
) {
    let monsters = game.get_alive_monsters();
    for m in monsters {
        game.action_queue.push_bot(DamageAction::from_player(
            if info.upgraded {
                upgraded_base_damage
            } else {
                unupgraded_base_damage
            },
            &game.player,
            game.get_creature(m),
            m,
        ));
    }
}

pub fn push_aoe_status(game: &mut Game, status: Status, amount: i32) {
    let monsters = game.get_alive_monsters();
    for m in monsters {
        game.action_queue.push_bot(GainStatusAction {
            status,
            amount,
            target: m,
        });
    }
}

pub fn strike_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    push_damage(game, target, info, 6, 9);
}

pub fn bash_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    push_damage(game, target, info, 8, 10);
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Vulnerable,
        amount: if info.upgraded { 3 } else { 2 },
        target: target.unwrap(),
    });
}

pub fn pommel_strike_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    push_damage(game, target, info, 9, 10);
    game.action_queue
        .push_bot(DrawAction(if info.upgraded { 2 } else { 1 }));
}

pub fn twin_strike_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    for _ in 0..2 {
        push_damage(game, target, info, 5, 7);
    }
}

pub fn clothesline_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    push_damage(game, target, info, 12, 14);
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Weak,
        amount: if info.upgraded { 3 } else { 2 },
        target: target.unwrap(),
    });
}

pub fn cleave_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    push_aoe_damage(game, info, 8, 11);
}

pub fn thunderclap_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    push_aoe_damage(game, info, 4, 7);
    push_aoe_status(game, Status::Vulnerable, 1);
}

pub fn searing_blow_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    let n = info.upgrade_count;
    game.action_queue.push_bot(DamageAction::from_player(
        n * (n + 7) / 2 + 12,
        &game.player,
        game.get_creature(target.unwrap()),
        target.unwrap(),
    ));
}

pub fn whirlwind_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    for _ in 0..game.energy {
        push_aoe_damage(game, info, 5, 8);
    }
}

pub fn rampage_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    let damage = if info.upgraded { 8 } else { 5 } * info.times_played + 8;
    push_damage(game, target, info, damage, damage);
}

pub fn swift_strike_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    push_damage(game, target, info, 7, 10);
}

pub fn flash_of_steel_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    push_damage(game, target, info, 3, 6);
    game.action_queue.push_bot(DrawAction(1));
}

pub fn dramatic_entrance_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    push_aoe_damage(game, info, 8, 12);
}

pub fn mind_blast_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    let damage = game.draw_pile.len() as i32;
    push_damage(game, target, info, damage, damage);
}

pub fn debug_kill_behavior(game: &mut Game, target: Option<CreatureRef>, _: CardPlayInfo) {
    game.action_queue.push_bot(DamageAction::from_player(
        9999,
        &game.player,
        game.get_creature(target.unwrap()),
        target.unwrap(),
    ));
}

#[cfg(test)]
mod tests {
    use crate::{
        cards::{CardClass, new_card, new_card_upgraded},
        game::{GameBuilder, Move},
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
        let mut g = GameBuilder::default()
            .add_card_upgraded(CardClass::Strike)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 9);
    }

    #[test]
    fn test_bash() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Bash)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 8);
        assert_eq!(g.monsters[0].creature.statuses.len(), 1);
        assert_eq!(
            g.monsters[0].creature.statuses.get(&Status::Vulnerable),
            Some(&2)
        );
    }

    #[test]
    fn test_upgraded_pommel_strike() {
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
    fn test_cleave() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Cleave, 2)
            .add_monster(NoopMonster())
            .add_monster(NoopMonster())
            .build_combat();
        let hp0 = g.monsters[0].creature.cur_hp;
        g.monsters[1].creature.cur_hp = 4;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 8);
        assert_eq!(g.monsters[1].creature.cur_hp, 0);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 16);
        assert_eq!(g.monsters[1].creature.cur_hp, 0);
    }

    #[test]
    fn test_thunderclap() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Thunderclap, 2)
            .add_monster(NoopMonster())
            .add_monster(NoopMonster())
            .build_combat();
        let hp0 = g.monsters[0].creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 4);
        assert_eq!(g.monsters[1].creature.cur_hp, hp0 - 4);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.monsters[0].creature.cur_hp, hp0 - 10);
        assert_eq!(g.monsters[1].creature.cur_hp, hp0 - 10);
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
            .add_monster(NoopMonster())
            .add_monster(NoopMonster())
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
        let mut g = GameBuilder::default()
            .add_card(CardClass::DebugKill)
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert!(!g.monsters[0].creature.is_alive());
    }

    #[test]
    fn test_flash_of_steel_finesse_infinite() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Finesse)
            .add_card(CardClass::FlashOfSteel)
            .add_monster(NoopMonster())
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
