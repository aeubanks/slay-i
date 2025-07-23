use crate::{
    action::Action,
    actions::{discard_hand::DiscardHandAction, exhaust_card::ExhaustCardAction},
    game::Game,
};

pub struct EndOfTurnDiscardAction();

impl Action for EndOfTurnDiscardAction {
    fn run(&self, game: &mut Game) {
        game.action_queue.push_top(DiscardHandAction());
        let mut indexes_to_exhaust = Vec::new();
        for (i, c) in game.hand.iter().enumerate() {
            if c.borrow().class.is_ethereal() {
                indexes_to_exhaust.push(i);
            }
        }
        for i in indexes_to_exhaust.into_iter().rev() {
            game.action_queue.push_top(ExhaustCardAction {
                card: game.hand.remove(i),
            });
        }
    }
}

impl std::fmt::Debug for EndOfTurnDiscardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "end of turn discard")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cards::{CardClass, new_card},
        game::{GameBuilder, Move},
    };

    #[test]
    fn test_ethereal() {
        let mut g = GameBuilder::default()
            .add_cards(new_card(CardClass::GhostlyArmor), 3)
            .add_cards(new_card(CardClass::Strike), 1)
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_eq!(g.exhaust_pile.len(), 3);
    }
}
