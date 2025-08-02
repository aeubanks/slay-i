use rand::Rng;

use crate::{action::Action, card::CardRef, game::Game};

pub struct ShuffleCardIntoDrawAction(pub CardRef);

impl Action for ShuffleCardIntoDrawAction {
    fn run(&self, game: &mut Game) {
        // cannot shuffle card on top of draw pile unless empty
        if game.draw_pile.is_empty() {
            game.draw_pile.push(self.0.clone());
        } else {
            let i = game.rng.random_range(0..game.draw_pile.len());
            game.draw_pile.insert(i, self.0.clone());
        }
    }
}

impl std::fmt::Debug for ShuffleCardIntoDrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shuffle card into draw: {:?}", self.0.borrow())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::shuffle_card_into_draw::ShuffleCardIntoDrawAction,
        cards::{CardClass, new_card},
        game::GameBuilder,
    };

    #[test]
    fn test_shuffle_into_draw_non_empty() {
        let mut g = GameBuilder::default().build_combat();
        g.draw_pile.push(new_card(CardClass::Strike));
        g.run_action(ShuffleCardIntoDrawAction(new_card(CardClass::Defend)));
        assert_eq!(g.draw_pile[0].borrow().class, CardClass::Defend);
        assert_eq!(g.draw_pile[1].borrow().class, CardClass::Strike);
    }

    #[test]
    fn test_shuffle_into_draw_non_empty_2() {
        let mut found_at_0 = false;
        let mut found_at_1 = false;
        for _ in 0..500 {
            let mut g = GameBuilder::default().build_combat();
            g.draw_pile.push(new_card(CardClass::Strike));
            g.draw_pile.push(new_card(CardClass::Strike));
            g.run_action(ShuffleCardIntoDrawAction(new_card(CardClass::Defend)));
            found_at_0 |= g.draw_pile[0].borrow().class == CardClass::Defend;
            found_at_1 |= g.draw_pile[1].borrow().class == CardClass::Defend;
            assert_eq!(g.draw_pile[2].borrow().class, CardClass::Strike);
            if found_at_0 && found_at_1 {
                break;
            }
        }
        assert!(found_at_0 && found_at_1);
    }

    #[test]
    fn test_shuffle_into_draw_empty() {
        let mut g = GameBuilder::default().build_combat();
        g.run_action(ShuffleCardIntoDrawAction(new_card(CardClass::Defend)));
        assert_eq!(g.draw_pile.len(), 1);
        assert_eq!(g.draw_pile[0].borrow().class, CardClass::Defend);
    }
}
