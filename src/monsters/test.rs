use crate::{
    actions::{damage::DamageAction, gain_status::GainStatusAction},
    game::{CreatureRef, Rand},
    monster::{Intent, MonsterBehavior, MonsterInfo},
    queue::ActionQueue,
    status::Status,
};

pub struct NoopMonster {
    max_hp: i32,
}

impl NoopMonster {
    pub fn new() -> Self {
        Self { max_hp: 500 }
    }
    #[allow(dead_code)]
    pub fn with_hp(hp: i32) -> Self {
        Self { max_hp: hp }
    }
}

impl MonsterBehavior for NoopMonster {
    fn name(&self) -> &'static str {
        "noop"
    }
    fn hp_range(&self) -> (i32, i32) {
        (self.max_hp, self.max_hp)
    }
    fn roll_next_action(&mut self, _r: &mut Rand, _info: &MonsterInfo) {}
    fn get_intent(&self) -> Intent {
        Intent::Sleep
    }
    fn take_turn(&mut self, _: CreatureRef, _: &mut ActionQueue) {}
}

pub struct AttackMonster {
    attack: i32,
    attack_count: i32,
    max_hp: i32,
}

#[allow(dead_code)]
impl AttackMonster {
    pub fn new(attack: i32) -> Self {
        Self {
            attack,
            attack_count: 1,
            max_hp: 100,
        }
    }
    pub fn with_hp(attack: i32, max_hp: i32) -> Self {
        Self {
            attack,
            attack_count: 1,
            max_hp,
        }
    }
    pub fn with_attack_count(attack: i32, attack_count: i32) -> Self {
        Self {
            attack,
            attack_count,
            max_hp: 50,
        }
    }
}

impl MonsterBehavior for AttackMonster {
    fn name(&self) -> &'static str {
        "attack"
    }
    fn hp_range(&self) -> (i32, i32) {
        (self.max_hp, self.max_hp)
    }
    fn roll_next_action(&mut self, _r: &mut Rand, _info: &MonsterInfo) {}
    fn get_intent(&self) -> Intent {
        Intent::Attack(self.attack, 1)
    }
    fn take_turn(&mut self, this: CreatureRef, queue: &mut ActionQueue) {
        for _ in 0..self.attack_count {
            queue.push_bot(DamageAction::from_monster(self.attack, this));
        }
    }
}

pub struct IntentMonster {
    intent: Intent,
}

#[allow(dead_code)]
impl IntentMonster {
    pub fn new(intent: Intent) -> Self {
        Self { intent }
    }
}

impl MonsterBehavior for IntentMonster {
    fn name(&self) -> &'static str {
        "intent"
    }
    fn hp_range(&self) -> (i32, i32) {
        (50, 50)
    }
    fn roll_next_action(&mut self, _r: &mut Rand, _info: &MonsterInfo) {}
    fn get_intent(&self) -> Intent {
        self.intent
    }
    fn take_turn(&mut self, _: CreatureRef, _: &mut ActionQueue) {}
}

#[allow(dead_code)]
pub struct ApplyStatusMonster {
    pub status: Status,
    pub amount: i32,
}

impl MonsterBehavior for ApplyStatusMonster {
    fn name(&self) -> &'static str {
        "apply-vuln"
    }
    fn hp_range(&self) -> (i32, i32) {
        (100, 100)
    }
    fn roll_next_action(&mut self, _r: &mut Rand, _info: &MonsterInfo) {}
    fn get_intent(&self) -> Intent {
        Intent::Debuff
    }
    fn take_turn(&mut self, _: CreatureRef, queue: &mut ActionQueue) {
        queue.push_bot(GainStatusAction {
            status: self.status,
            amount: self.amount,
            target: CreatureRef::player(),
        });
    }
}
