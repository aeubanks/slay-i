use crate::queue::ActionQueue;

pub trait Relic {
    fn pre_combat(&mut self, _queue: &mut ActionQueue) {}
    fn combat_start_pre_draw(&mut self, _queue: &mut ActionQueue) {}
    fn combat_start_post_draw(&mut self, _queue: &mut ActionQueue) {}
    fn turn_end(&mut self, _queue: &mut ActionQueue) {}
    fn combat_finish(&mut self, _queue: &mut ActionQueue) {}
}
