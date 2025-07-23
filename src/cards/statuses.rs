use crate::{
    actions::damage::DamageAction,
    game::{CreatureRef, Game},
};

pub fn burn_behavior(game: &mut Game) {
    game.action_queue
        .push_bot(DamageAction::thorns(2, CreatureRef::player()));
}

pub fn burn_plus_behavior(game: &mut Game) {
    game.action_queue
        .push_bot(DamageAction::thorns(4, CreatureRef::player()));
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::block::BlockAction,
        cards::{CardClass, new_card},
        game::{CreatureRef, GameBuilder, Move},
    };

    #[test]
    fn test_playable() {
        let mut g = GameBuilder::default().build_combat();
        g.hand.push(new_card(CardClass::Wound));
        g.hand.push(new_card(CardClass::Slimed));
        g.hand.push(new_card(CardClass::Dazed));
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
        assert_eq!(g.player.creature.cur_hp, 48);
    }

    #[test]
    fn test_burn_plus() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::BurnPlus)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction {
            target: CreatureRef::player(),
            amount: 1,
        });
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.cur_hp, 47);
    }

    #[test]
    #[should_panic]
    fn test_crash_on_play_unplayable_status() {
        let mut g = GameBuilder::default().build_combat();
        g.hand.push(new_card(CardClass::Wound));
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
    }
}
