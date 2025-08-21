use crate::{
    action::Action,
    actions::place_card_in_hand::PlaceCardInHandAction,
    cards::CardType,
    game::{Game, GameState},
};

pub struct ChooseCardInDrawToPlaceInHandAction(pub CardType);

enum Count {
    Zero,
    One(usize),
    Many,
}

impl Action for ChooseCardInDrawToPlaceInHandAction {
    fn run(&self, game: &mut Game) {
        let mut count = Count::Zero;
        for (i, c) in game.draw_pile.iter().enumerate() {
            if c.borrow().class.ty() == self.0 {
                match count {
                    Count::Zero => count = Count::One(i),
                    Count::One(_) => {
                        count = Count::Many;
                        break;
                    }
                    Count::Many => unreachable!(),
                }
            }
        }
        match count {
            Count::Zero => unreachable!(),
            Count::One(i) => game
                .action_queue
                .push_top(PlaceCardInHandAction(game.draw_pile.remove(i))),
            Count::Many => game.state = GameState::FetchCardFromDraw(self.0),
        }
    }
}

impl std::fmt::Debug for ChooseCardInDrawToPlaceInHandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose card in draw to place in hand ({:?})", self.0)
    }
}
