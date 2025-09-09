use crate::{
    actions::{damage::DamageAction, gain_status::GainStatusAction},
    game::{CreatureRef, Game},
    status::Status,
};

pub fn regret_behavior(game: &mut Game) {
    let hand_size = game.hand.len();
    game.action_queue.push_top(DamageAction::lose_hp(
        hand_size as i32,
        CreatureRef::player(),
    ));
}

pub fn decay_behavior(game: &mut Game) {
    game.action_queue
        .push_top(DamageAction::thorns_rupture(2, CreatureRef::player()));
}

pub fn doubt_behavior(game: &mut Game) {
    game.action_queue.push_top(GainStatusAction {
        status: Status::Weak,
        amount: 1,
        target: CreatureRef::player(),
    });
}

pub fn shame_behavior(game: &mut Game) {
    game.action_queue.push_top(GainStatusAction {
        status: Status::Frail,
        amount: 1,
        target: CreatureRef::player(),
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::block::BlockAction,
        cards::CardClass,
        game::{GameBuilder, Move},
        relic::RelicClass,
        status::Status,
    };

    #[test]
    fn test_playable() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::AscendersBane);
        assert_eq!(g.valid_moves(), vec![Move::EndTurn]);
    }

    #[test]
    #[should_panic]
    fn test_crash_on_play_unplayable_curse() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::AscendersBane);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
    }

    #[test]
    fn test_regret() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 4)
            .add_card(CardClass::Regret)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction::player_flat_amount(4));
        assert_eq!(g.player.creature.cur_hp, 50);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.cur_hp, 45);
    }

    #[test]
    fn test_regret2() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 3)
            .add_cards(CardClass::Regret, 2)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction::player_flat_amount(4));
        assert_eq!(g.player.creature.cur_hp, 50);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.cur_hp, 40);
    }

    #[test]
    fn test_decay() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Decay)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction::player_flat_amount(1));
        assert_eq!(g.player.creature.cur_hp, 50);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.cur_hp, 49);
    }

    #[test]
    fn test_doubt() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 4)
            .add_card(CardClass::Doubt)
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.get_status(Status::Weak), Some(1));
    }

    #[test]
    fn test_doubt2() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 4)
            .add_card(CardClass::Doubt)
            .add_player_status(Status::Weak, 1)
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.get_status(Status::Weak), Some(1));
    }

    #[test]
    fn test_shame() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 4)
            .add_card(CardClass::Shame)
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.get_status(Status::Frail), Some(1));
    }

    #[test]
    fn test_normality() {
        let mut g = GameBuilder::default().build_combat();
        g.energy = 10;
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Normality);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.valid_moves(), vec![Move::EndTurn]);
        g.hand.pop();
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::EndTurn,
                Move::PlayCard {
                    card_index: 0,
                    target: None
                }
            ]
        );
        g.make_move(Move::EndTurn);
        g.energy = 10;
        g.hand.clear();
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Defend);
        g.add_card_to_hand(CardClass::Defend);
        assert_eq!(g.valid_moves().len(), 6);
        for _ in 0..4 {
            g.make_move(Move::PlayCard {
                card_index: 0,
                target: None,
            });
        }
        assert_eq!(g.valid_moves().len(), 2);
        g.add_card_to_hand(CardClass::Normality);
        assert_eq!(g.valid_moves().len(), 1);
    }

    #[test]
    fn test_necronomicurse() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::BlueCandle)
            .build_combat();
        g.add_card_to_hand(CardClass::Necronomicurse);
        g.play_card(CardClass::TrueGrit, None);
        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.hand[0].borrow().class, CardClass::Necronomicurse);
        assert_eq!(g.exhaust_pile.len(), 1);
        assert_eq!(g.exhaust_pile[0].borrow().class, CardClass::Necronomicurse);
    }

    #[test]
    fn test_parasite() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Parasite)
            .build_combat();
        let max_hp = g.player.creature.max_hp;
        g.player.creature.cur_hp = max_hp - 1;
        g.remove_card_from_master_deck(0);
        assert_eq!(g.player.creature.max_hp, max_hp - 3);
        assert_eq!(g.player.creature.cur_hp, max_hp - 3);
    }
}
