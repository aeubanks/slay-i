use crate::action::Action;

#[derive(Default)]
pub struct ActionQueue {
    queue: Vec<Box<dyn Action>>,
    debug: bool,
}

impl ActionQueue {
    pub fn set_debug(&mut self) {
        self.debug = true;
    }
    pub fn push_bot<A: Action + 'static>(&mut self, a: A) {
        if self.debug {
            println!("push_bot {a:?}");
        }
        self.queue.insert(0, Box::new(a));
    }
    pub fn push_top<A: Action + 'static>(&mut self, a: A) {
        if self.debug {
            println!("push_top {a:?}");
        }
        self.queue.push(Box::new(a));
    }
    pub fn pop(&mut self) -> Option<Box<dyn Action>> {
        let a = self.queue.pop();
        if let Some(a) = &a
            && self.debug
        {
            println!("pop {a:?}");
        }
        a
    }
}
