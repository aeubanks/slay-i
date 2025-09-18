use crate::{
    actions::damage::DamageAction,
    game::{CreatureRef, Game},
};

pub fn burn_behavior(game: &mut Game) {
    game.action_queue
        .push_bot(DamageAction::thorns_rupture(2, CreatureRef::player()));
}

pub fn burn_plus_behavior(game: &mut Game) {
    game.action_queue
        .push_bot(DamageAction::thorns_rupture(4, CreatureRef::player()));
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::block::BlockAction,
        cards::CardClass,
        game::{GameBuilder, Move},
    };

    #[test]
    fn test_playable() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Wound);
        g.add_card_to_hand(CardClass::Slimed);
        g.add_card_to_hand(CardClass::Dazed);
        assert_eq!(
            g.valid_moves(),
            vec![
                Move::EndTurn,
                Move::PlayCard {
                    card_index: 1,
                    target: None
                }
            ]
        );
        g.make_move(Move::PlayCard {
            card_index: 1,
            target: None,
        });
        assert_eq!(g.energy, 2);
        assert_eq!(g.exhaust_pile.len(), 1);
    }

    #[test]
    fn test_burn() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Burn)
            .set_player_hp(50)
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.cur_hp, 48);
    }

    #[test]
    fn test_burn_plus() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::BurnPlus)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction::player_flat_amount(1));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.cur_hp, 47);
    }

    #[test]
    fn test_void() {
        {
            let mut g = GameBuilder::default()
                .add_cards(CardClass::Void, 2)
                .build_combat();
            assert_eq!(g.energy, 1);
            g.make_move(Move::EndTurn);
            assert_eq!(g.exhaust_pile.len(), 2);
        }
        {
            let g = GameBuilder::default()
                .add_cards(CardClass::Void, 4)
                .build_combat();
            assert_eq!(g.energy, 0);
        }
    }

    #[test]
    #[should_panic]
    fn test_crash_on_play_unplayable_status() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Wound);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
    }
}
