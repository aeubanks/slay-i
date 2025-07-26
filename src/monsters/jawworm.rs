use rand::Rng;

use crate::{
    actions::{block::BlockAction, damage::DamageAction, gain_status::GainStatusAction},
    creature::Creature,
    game::{CreatureRef, Rand},
    monster::{Intent, MonsterBehavior, MonsterInfo},
    move_history::MoveHistory,
    player::Player,
    queue::ActionQueue,
    status::Status,
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Action {
    None,
    Bellow,
    Chomp,
    Thrash,
}

pub struct JawWorm {
    action: Action,
    history: MoveHistory<Action>,
}

impl JawWorm {
    pub fn new() -> Self {
        Self {
            action: Action::None,
            history: MoveHistory::new(),
        }
    }
}

impl MonsterBehavior for JawWorm {
    fn name(&self) -> &'static str {
        "jawworm"
    }
    fn roll_hp(&self, r: &mut Rand) -> i32 {
        r.random_range(42..=46)
    }

    fn take_turn(
        &mut self,
        queue: &mut ActionQueue,
        player: &Player,
        this: &Creature,
        this_ref: CreatureRef,
    ) {
        match self.action {
            Action::Chomp => {
                queue.push_bot(DamageAction::from_monster(12, player, this, this_ref));
            }
            Action::Thrash => {
                queue.push_bot(DamageAction::from_monster(7, player, this, this_ref));

                queue.push_bot(BlockAction::monster(this_ref, 5));
            }
            Action::Bellow => {
                queue.push_bot(GainStatusAction {
                    status: Status::Strength,
                    amount: 5,
                    target: this_ref,
                });
                queue.push_bot(BlockAction::monster(this_ref, 9));
            }
            Action::None => unreachable!(),
        }
    }
    fn roll_next_action(&mut self, r: &mut Rand, _info: &MonsterInfo) {
        let next = if self.action == Action::None {
            Action::Chomp
        } else {
            let num = r.random_range(0..100);
            if num < 25 {
                if self.history.last(Action::Chomp) {
                    if r.random_bool(0.5625) {
                        Action::Bellow
                    } else {
                        Action::Thrash
                    }
                } else {
                    Action::Chomp
                }
            } else if num < 55 {
                if self.history.last_two(Action::Thrash) {
                    if r.random_bool(0.357) {
                        Action::Chomp
                    } else {
                        Action::Bellow
                    }
                } else {
                    Action::Thrash
                }
            } else if self.history.last(Action::Bellow) {
                if r.random_bool(0.416) {
                    Action::Chomp
                } else {
                    Action::Thrash
                }
            } else {
                Action::Bellow
            }
        };
        self.history.add(next);
        self.action = next;
    }

    fn get_intent(&self) -> Intent {
        match self.action {
            Action::None => unreachable!(),
            Action::Bellow => Intent::DefendBuff,
            Action::Chomp => Intent::Attack(12, 1),
            Action::Thrash => Intent::AttackDefend(7, 1),
        }
    }
}
