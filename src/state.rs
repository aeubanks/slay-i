use crate::{game::Game, step::Step};

use std::fmt::Debug;

pub trait GameState: Debug {
    fn run(&self, _: &mut Game) {}
    fn valid_steps(&self, _: &Game) -> Option<Vec<Box<dyn Step>>> {
        None
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct NoopStep;

impl Step for NoopStep {
    fn run(&self, _: &mut Game) {}

    fn description(&self, _: &Game) -> String {
        "noop".to_owned()
    }
}

#[derive(Default)]
pub struct GameStateManager {
    stack: Vec<Box<dyn GameState>>,
    debug: bool,
}

impl std::fmt::Debug for GameStateManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "state stack: {:?}", self.stack)
    }
}

impl GameStateManager {
    pub fn clear(&mut self) {
        self.stack.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn set_debug(&mut self) {
        self.debug = true;
    }
    pub fn push_state<T: GameState + 'static>(&mut self, state: T) {
        if self.debug {
            println!("push_state {:?}", state);
        }
        self.stack.push(Box::new(state));
    }
    pub fn pop_state(&mut self) -> Option<Box<dyn GameState>> {
        let state = self.stack.pop();
        if self.debug
            && let Some(s) = &state
        {
            println!("pop_state {:?} ({:?})", s, self.stack);
        }
        state
    }
}
