use crate::{
    actions::{damage::DamageAction, gain_status::GainStatusAction},
    creature::Creature,
    game::{CreatureRef, Rand},
    monster::{Intent, MonsterBehavior, MonsterInfo},
    player::Player,
    queue::ActionQueue,
    status::Status,
};

pub struct NoopMonster {
    max_hp: i32,
}

impl NoopMonster {
    pub fn new() -> Self {
        Self { max_hp: 100 }
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
    fn roll_hp(&self, _r: &mut Rand) -> i32 {
        self.max_hp
    }
    fn roll_next_action(&mut self, _r: &mut Rand, _info: &MonsterInfo) {}
    fn get_intent(&self) -> Intent {
        Intent::Sleep
    }
    fn take_turn(&mut self, _: &mut ActionQueue, _: &Player, _: &Creature, _: CreatureRef) {}
}

pub struct AttackMonster {
    attack: i32,
    max_hp: i32,
}

#[allow(dead_code)]
impl AttackMonster {
    pub fn new(attack: i32) -> Self {
        Self {
            attack,
            max_hp: 100,
        }
    }
    pub fn with_hp(attack: i32, max_hp: i32) -> Self {
        Self { attack, max_hp }
    }
}

impl MonsterBehavior for AttackMonster {
    fn name(&self) -> &'static str {
        "attack"
    }
    fn roll_hp(&self, _r: &mut Rand) -> i32 {
        self.max_hp
    }
    fn roll_next_action(&mut self, _r: &mut Rand, _info: &MonsterInfo) {}
    fn get_intent(&self) -> Intent {
        Intent::Attack(self.attack, 1)
    }
    fn take_turn(
        &mut self,
        queue: &mut ActionQueue,
        player: &Player,
        this: &Creature,
        this_ref: CreatureRef,
    ) {
        queue.push_bot(DamageAction::from_monster(
            self.attack,
            player,
            this,
            this_ref,
        ));
    }
}

pub struct ApplyStatusMonster {
    pub status: Status,
    pub amount: i32,
}

impl MonsterBehavior for ApplyStatusMonster {
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
            status: self.status,
            amount: self.amount,
            target: CreatureRef::player(),
        });
    }
}
