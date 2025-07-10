use std::collections::HashMap;

use crate::{
    actions::{draw::DrawAction, gain_status::GainStatusAction, lose_hp::LoseHPAction},
    game::CreatureRef,
    queue::ActionQueue,
    status::Status,
};

#[derive(Default)]
pub struct Creature {
    pub name: &'static str,
    pub max_hp: i32,
    pub cur_hp: i32,
    pub block: i32,
    pub statuses: HashMap<Status, i32>,
}

impl Creature {
    pub fn new(name: &'static str, max_hp: i32) -> Self {
        Self {
            name,
            max_hp,
            cur_hp: max_hp,
            block: 0,
            statuses: Default::default(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.cur_hp > 0
    }

    pub fn heal(&mut self, amount: i32) {
        self.cur_hp += amount;
        if self.cur_hp > self.max_hp {
            self.cur_hp = self.max_hp;
        }
    }

    pub fn increase_max_hp(&mut self, amount: i32) {
        self.max_hp += amount;
    }

    pub fn trigger_statuses_turn_begin(&mut self, this: CreatureRef, queue: &mut ActionQueue) {
        if let Some(v) = self.statuses.get(&Status::DemonForm) {
            queue.push_bot(GainStatusAction {
                status: Status::Strength,
                amount: *v,
                target: this,
            });
        }
        if let Some(v) = self.statuses.get(&Status::Brutality) {
            queue.push_bot(DrawAction(*v));
            queue.push_bot(LoseHPAction {
                target: this,
                amount: *v,
            });
        }
    }

    pub fn trigger_statuses_turn_end(&mut self, _this: CreatureRef, _queue: &mut ActionQueue) {}
    pub fn trigger_statuses_round_end(&mut self, this: CreatureRef, queue: &mut ActionQueue) {
        for s in self.statuses.keys() {
            if s.decays() {
                queue.push_bot(GainStatusAction {
                    status: *s,
                    amount: -1,
                    target: this,
                });
            }
        }
    }
}
