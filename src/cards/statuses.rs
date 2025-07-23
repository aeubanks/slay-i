#[cfg(test)]
mod tests {
    use crate::{
        cards::{CardClass, new_card},
        game::{GameBuilder, Move},
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
