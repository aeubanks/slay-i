use crate::{action::Action, game::Game};

#[allow(dead_code)]
pub struct NoopAction();

impl Action for NoopAction {
    fn run(&self, _: &mut Game) {}
}

impl std::fmt::Debug for NoopAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "noop")
    }
}
