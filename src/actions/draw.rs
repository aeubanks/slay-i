use crate::{
    action::Action, actions::shuffle_discard_on_top_of_draw::ShuffleDiscardOnTopOfDrawAction,
    game::Game,
};

pub struct DrawAction(pub i32);

impl Action for DrawAction {
    fn run(&self, game: &mut Game) {
        let discard_size = game.discard_pile.len() as i32;
        let draw_size = game.draw_pile.len() as i32;
        let hand_size = game.hand.len() as i32;

        if draw_size == 0 && discard_size == 0 {
            return;
        }

        let mut amount = self.0.min(10 - hand_size);
        if amount == 0 {
            return;
        }

        if amount > draw_size {
            game.action_queue.push_top(DrawAction(amount - draw_size));
            game.action_queue
                .push_top(ShuffleDiscardOnTopOfDrawAction());
            amount = draw_size;
        }

        for _ in 0..amount {
            game.hand.push(game.draw_pile.pop().unwrap());
        }
    }
}

impl std::fmt::Debug for DrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "draw {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::draw::DrawAction,
        cards::{CardClass, card},
        game::GameBuilder,
    };

    #[test]
    fn test_shuffle() {
        let mut gb = GameBuilder::default();
        for _ in 0..12 {
            gb = gb.add_card(card(CardClass::Strike));
        }
        let mut g = gb.build();

        assert_eq!(g.hand.len(), 5);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 7);

        g.run_action(DrawAction(1));

        assert_eq!(g.hand.len(), 6);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 6);

        g.run_action(DrawAction(5));

        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 2);

        g.discard_pile.push(g.hand.pop().unwrap());
        g.discard_pile.push(g.hand.pop().unwrap());

        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.discard_pile.len(), 2);
        assert_eq!(g.draw_pile.len(), 2);

        g.run_action(DrawAction(5));

        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.discard_pile.len(), 2);
        assert_eq!(g.draw_pile.len(), 0);

        g.discard_pile.push(g.hand.pop().unwrap());
        g.discard_pile.push(g.hand.pop().unwrap());

        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.discard_pile.len(), 4);
        assert_eq!(g.draw_pile.len(), 0);

        g.run_action(DrawAction(1));

        assert_eq!(g.hand.len(), 9);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 3);

        g.draw_pile.clear();

        g.run_action(DrawAction(1));

        assert_eq!(g.hand.len(), 9);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);

        g.discard_pile.push(g.hand.pop().unwrap());
        g.discard_pile.push(g.hand.pop().unwrap());
        g.draw_pile.push(g.hand.pop().unwrap());
        g.draw_pile.push(g.hand.pop().unwrap());

        assert_eq!(g.hand.len(), 5);
        assert_eq!(g.discard_pile.len(), 2);
        assert_eq!(g.draw_pile.len(), 2);

        g.run_action(DrawAction(3));

        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 1);
    }
}
