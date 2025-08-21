use crate::{
    action::Action,
    cards::CardClass,
    game::{Game, GameState},
};

pub struct ExhumeAction();

enum Count {
    Zero,
    One(usize),
    Many,
}

impl Action for ExhumeAction {
    fn run(&self, game: &mut Game) {
        if game.hand_is_full() {
            return;
        }
        let mut count = Count::Zero;
        for (i, c) in game.exhaust_pile.iter().enumerate() {
            if c.borrow().class != CardClass::Exhume {
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
            Count::Zero => {}
            Count::One(i) => game.hand.push(game.exhaust_pile.remove(i)),
            Count::Many => game.state = GameState::Exhume,
        }
    }
}

impl std::fmt::Debug for ExhumeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exhume")
    }
}
