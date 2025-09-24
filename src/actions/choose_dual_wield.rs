use crate::{
    action::Action, actions::dual_wield::DualWieldAction, card::CardRef, cards::CardType,
    game::Game, state::GameState,
};

pub struct ChooseDualWieldAction(pub i32);

enum Count {
    Zero,
    One(usize),
    Many,
}

pub fn can_dual_wield(c: &CardRef) -> bool {
    matches!(c.borrow().class.ty(), CardType::Attack | CardType::Power)
}

impl Action for ChooseDualWieldAction {
    fn run(&self, game: &mut Game) {
        let mut count = Count::Zero;
        for (i, c) in game.hand.iter().enumerate() {
            if can_dual_wield(c) {
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
            Count::One(i) => {
                game.action_queue.push_top(DualWieldAction {
                    card: game.hand.remove(i),
                    amount: self.0,
                    destroy_original: false,
                });
            }
            Count::Many => game.state.push_state(GameState::DualWield(self.0)),
        }
    }
}

impl std::fmt::Debug for ChooseDualWieldAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose dual wield")
    }
}
