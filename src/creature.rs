use std::collections::HashMap;

use crate::{
    actions::{
        block::BlockAction, damage::DamageAction, damage_all_monsters::DamageAllMonstersAction,
        draw::DrawAction, gain_energy::GainEnergyAction, gain_status::GainStatusAction,
        heal::HealAction, play_card::PlayCardAction, reduce_status::ReduceStatusAction,
        remove_status::RemoveStatusAction,
    },
    cards::CardType,
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

    pub fn start_of_turn_lose_block(&mut self) {
        if self.statuses.contains_key(&Status::Barricade) {
            return;
        }
        self.block = 0;
    }

    pub fn trigger_statuses_on_card_played(
        &mut self,
        queue: &mut ActionQueue,
        card_queue: &mut Vec<PlayCardAction>,
        play: &PlayCardAction,
    ) {
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
        if !play.is_duplicated {
            if self.statuses.contains_key(&Status::Duplication) {
                queue.push_top(ReduceStatusAction {
                    status: Status::Duplication,
                    amount: 1,
                    target: CreatureRef::player(),
                });
                card_queue.push(PlayCardAction {
                    card: play.card.clone(),
                    target: play.target,
                    is_duplicated: true,
                    energy: play.energy,
                    force_exhaust: false,
                    free: true,
                });
            }
            if self.statuses.contains_key(&Status::DoubleTap)
                && play.card.borrow().class.ty() == CardType::Attack
            {
                queue.push_top(ReduceStatusAction {
                    status: Status::DoubleTap,
                    amount: 1,
                    target: CreatureRef::player(),
                });
                card_queue.push(PlayCardAction {
                    card: play.card.clone(),
                    target: play.target,
                    is_duplicated: true,
                    energy: play.energy,
                    force_exhaust: false,
                    free: true,
                });
            }
            if self.statuses.contains_key(&Status::PenNib)
                && play.card.borrow().class.ty() == CardType::Attack
            {
                queue.push_top(RemoveStatusAction {
                    status: Status::PenNib,
                    target: CreatureRef::player(),
                });
            }
            if let Some(v) = self.statuses.get(&Status::Rage)
                && play.card.borrow().class.ty() == CardType::Attack
            {
                queue.push_bot(BlockAction::player_flat_amount(*v));
            }
        }
    }

    pub fn trigger_statuses_turn_begin(&mut self, this: CreatureRef, queue: &mut ActionQueue) {
        if let Some(v) = self.statuses.get(&Status::Berserk) {
            queue.push_bot(GainEnergyAction(*v));
        }
        if let Some(v) = self.statuses.get(&Status::NextTurnBlock) {
            queue.push_bot(BlockAction::player_flat_amount(*v));
            queue.push_bot(RemoveStatusAction {
                status: Status::NextTurnBlock,
                target: this,
            });
        }
        for status in [Status::NextTurnBlock, Status::FlameBarrier] {
            if self.statuses.contains_key(&status) {
                queue.push_bot(RemoveStatusAction {
                    status,
                    target: this,
                });
            }
        }
    }

    pub fn trigger_statuses_turn_begin_post_draw(
        &mut self,
        this: CreatureRef,
        queue: &mut ActionQueue,
    ) {
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

    pub fn trigger_statuses_turn_end(&mut self, this: CreatureRef, queue: &mut ActionQueue) {
        if let Some(v) = self.statuses.get(&Status::CombustHPLoss) {
            queue.push_bot(DamageAction::lose_hp(*v, this));
        }
        if let Some(v) = self.statuses.get(&Status::CombustDamage) {
            queue.push_bot(DamageAllMonstersAction::thorns(*v));
        }
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
        if let Some(v) = self.statuses.get(&Status::RegenPlayer) {
            // yes, this is push_top
            queue.push_top(HealAction {
                target: this,
                amount: *v,
            });
            queue.push_top(ReduceStatusAction {
                status: Status::RegenPlayer,
                amount: 1,
                target: this,
            });
        }
        if let Some(v) = self.statuses.get(&Status::RegenMonster) {
            queue.push_bot(HealAction {
                target: this,
                amount: *v,
            });
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
        if self.statuses.contains_key(&Status::Rage) {
            queue.push_bot(RemoveStatusAction {
                status: Status::Rage,
                target: this,
            });
        }
    }
    pub fn trigger_statuses_round_end(&mut self, this: CreatureRef, queue: &mut ActionQueue) {
        for s in self.statuses.keys() {
            if s.decays() {
                queue.push_bot(ReduceStatusAction {
                    status: *s,
                    amount: 1,
                    target: this,
                });
            }
            if s.disappears_end_of_turn() {
                queue.push_bot(RemoveStatusAction {
                    status: *s,
                    target: this,
                });
            }
        }
    }
}
