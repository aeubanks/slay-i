use std::collections::HashMap;

use crate::{
    actions::{
        damage::DamageAction, damage_all_monsters::DamageAllMonstersAction, draw::DrawAction,
        gain_status::GainStatusAction,
    },
    card::Card,
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

    pub fn trigger_statuses_on_card_played(&mut self, queue: &mut ActionQueue, _: &Card) {
        for (p, p_next) in [
            (Status::Panache5, Status::Panache4),
            (Status::Panache4, Status::Panache3),
            (Status::Panache3, Status::Panache2),
            (Status::Panache2, Status::Panache1),
            (Status::Panache1, Status::Panache5),
        ] {
            if let Some(&v) = self.statuses.get(&p) {
                self.statuses.insert(p_next, v);
                self.statuses.remove_entry(&p);
                if p == Status::Panache1 {
                    queue.push_bot(DamageAllMonstersAction::thorns(v));
                }
                break;
            }
        }
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
            queue.push_bot(DamageAction::lose_hp(*v, this));
        }
    }

    pub fn trigger_statuses_turn_end(&mut self, _this: CreatureRef, queue: &mut ActionQueue) {
        if let Some(b) = self.statuses.get(&Status::Bomb1) {
            queue.push_bot(DamageAllMonstersAction::thorns(*b));
            self.statuses.remove(&Status::Bomb1);
        }
        if let Some(b) = self.statuses.get(&Status::Bomb2) {
            self.statuses.insert(Status::Bomb1, *b);
            self.statuses.remove(&Status::Bomb2);
        }
        if let Some(b) = self.statuses.get(&Status::Bomb3) {
            self.statuses.insert(Status::Bomb2, *b);
            self.statuses.remove(&Status::Bomb3);
        }
        for p in [
            Status::Panache4,
            Status::Panache3,
            Status::Panache2,
            Status::Panache1,
        ] {
            if let Some(v) = self.statuses.get(&p) {
                self.statuses.insert(Status::Panache5, *v);
                self.statuses.remove_entry(&p);
                break;
            }
        }
    }
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
