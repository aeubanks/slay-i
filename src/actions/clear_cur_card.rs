use crate::{action::Action, game::Game};

pub struct ClearCurCardAction();

impl Action for ClearCurCardAction {
    fn run(&self, g: &mut Game) {
        g.cur_card = None;
    }
}

impl std::fmt::Debug for ClearCurCardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "clear current card")
    }
}
