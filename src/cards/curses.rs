#[cfg(test)]
mod tests {
    use crate::{
        cards::{CardClass, new_card},
        game::{GameBuilder, Move},
    };

    #[test]
    fn test_playable() {
        let mut g = GameBuilder::default().build_combat();
        g.hand.push(new_card(CardClass::AscendersBane));
        assert_eq!(g.valid_moves(), vec![Move::EndTurn]);
    }

    #[test]
    #[should_panic]
    fn test_crash_on_play_unplayable_curse() {
        let mut g = GameBuilder::default().build_combat();
        g.hand.push(new_card(CardClass::AscendersBane));
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
    }
}
