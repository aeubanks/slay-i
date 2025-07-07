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

pub fn clothesline_behavior(game: &mut Game, target: Option<CreatureRef>, info: CardPlayInfo) {
    push_damage(game, target, info, 12, 14);
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Weak,
        amount: if info.upgraded { 3 } else { 2 },
        target: target.unwrap(),
    });
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
        cards::{CardClass, card, upgraded_card},
        game::{GameBuilder, Move},
        status::Status,
    };

    #[test]
    fn test_strike() {
        let mut g = GameBuilder::default()
            .add_card(card(CardClass::Strike))
            .build();
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
            .add_card(upgraded_card(CardClass::Strike))
            .build();
        let hp = g.monsters[0].creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 9);
    }

    #[test]
    fn test_bash() {
        let mut g = GameBuilder::default()
            .add_card(card(CardClass::Bash))
            .build();
        let hp = g.monsters[0].creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);
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
            gb = gb.add_card(upgraded_card(CardClass::PommelStrike));
        }
        let mut g = gb.build();
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
            gb = gb.add_card(card(CardClass::PommelStrike));
        }
        let mut g = gb.build();
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
    fn test_searing_blow() {
        for (upgrade_count, damage) in [(0, 12), (1, 16), (2, 21), (3, 27)] {
            let c = card(CardClass::SearingBlow);
            for _ in 0..upgrade_count {
                c.borrow_mut().upgrade();
            }
            let mut g = GameBuilder::default().add_card(c).build();
            let hp = g.monsters[0].creature.cur_hp;
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: Some(0),
            });
            assert_eq!(g.monsters[0].creature.cur_hp, hp - damage);
        }
    }

    #[test]
    fn test_debug_kill() {
        let mut g = GameBuilder::default()
            .add_card(card(CardClass::DebugKill))
            .build();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert!(!g.monsters[0].creature.is_alive());
    }

    #[test]
    #[should_panic]
    fn test_upgrade_crash() {
        let c = upgraded_card(CardClass::Strike);
        let mut c = c.borrow_mut();
        c.upgrade();
    }
}
