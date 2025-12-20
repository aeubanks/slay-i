use crate::{game::Game, step::Step};

use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Steps {
    pub steps: Vec<Box<dyn Step>>,
}

impl Steps {
    pub fn push<T: Step>(&mut self, step: T) {
        self.steps.push(Box::new(step));
    }
}

pub trait GameState: Debug {
    fn run(&self, _: &mut Game) {}
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        None
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct NoopStep;

impl Step for NoopStep {
    fn should_pop_state(&self) -> bool {
        true
    }
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
        self.push_boxed_state(Box::new(state));
    }
    pub fn push_boxed_state(&mut self, state: Box<dyn GameState>) {
        if self.debug {
            println!("push_state {:?}", state);
        }
        self.stack.push(state);
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
    pub fn peek(&self) -> &dyn GameState {
        self.stack.last().unwrap().as_ref()
    }
}
