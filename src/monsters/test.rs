use crate::{
    actions::gain_status::GainStatusAction,
    creature::Creature,
    game::{CreatureRef, Rand},
    monster::{Intent, MonsterBehavior, MonsterInfo},
    player::Player,
    queue::ActionQueue,
    status::Status,
};

pub struct NoopMonster();

impl MonsterBehavior for NoopMonster {
    fn name(&self) -> &'static str {
        "noop"
    }
    fn roll_hp(&self, _r: &mut Rand) -> i32 {
        100
    }
    fn roll_next_action(&mut self, _r: &mut Rand, _info: &MonsterInfo) {}
    fn get_intent(&self) -> Intent {
        Intent::Sleep
    }
    fn take_turn(&mut self, _: &mut ActionQueue, _: &Player, _: &Creature, _: CreatureRef) {}
}

pub struct ApplyVulnerableMonster();

impl MonsterBehavior for ApplyVulnerableMonster {
    fn name(&self) -> &'static str {
        "apply-vuln"
    }
    fn roll_hp(&self, _r: &mut Rand) -> i32 {
        100
    }
    fn roll_next_action(&mut self, _r: &mut Rand, _info: &MonsterInfo) {}
    fn get_intent(&self) -> Intent {
        Intent::Debuff
    }
    fn take_turn(&mut self, queue: &mut ActionQueue, _: &Player, _: &Creature, _: CreatureRef) {
        queue.push_bot(GainStatusAction {
            status: Status::Vulnerable,
            amount: 2,
            target: CreatureRef::player(),
        });
    }
}
