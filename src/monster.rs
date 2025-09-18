use crate::actions::damage::calculate_damage;
use crate::creature::Creature;
use crate::game::{CreatureRef, Game, Rand};
use crate::queue::ActionQueue;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Intent {
    Attack(i32, i32),
    AttackBuff(i32, i32),
    AttackDebuff(i32, i32),
    AttackDefend(i32, i32),
    Buff,
    Debuff,
    StrongDebuff,
    Defend,
    DefendBuff,
    DefendDebuff,
    Escape,
    Sleep,
    Stun,
}

impl Intent {
    pub fn is_attack(&self) -> bool {
        use Intent::*;
        matches!(
            self,
            Attack(..) | AttackBuff(..) | AttackDebuff(..) | AttackDefend(..)
        )
    }
    pub fn modify_damage(&mut self, this: CreatureRef, game: &Game) {
        use Intent::*;
        let d = match self {
            Attack(d, _) => d,
            AttackBuff(d, _) => d,
            AttackDebuff(d, _) => d,
            AttackDefend(d, _) => d,
            _ => {
                return;
            }
        };
        *d = calculate_damage(*d, this, CreatureRef::player(), game);
    }
}

#[allow(dead_code)]
pub struct MonsterInfo {
    pub num_monsters: usize,
}

pub trait MonsterBehavior {
    fn name(&self) -> &'static str;
    fn roll_hp(&self, r: &mut Rand) -> i32;
    fn pre_combat(&self, _queue: &mut ActionQueue, _this: CreatureRef) {}
    fn roll_next_action(&mut self, r: &mut Rand, info: &MonsterInfo);
    fn take_turn(&mut self, this: CreatureRef, queue: &mut ActionQueue);
    fn get_intent(&self) -> Intent;
}

pub struct Monster {
    pub creature: Creature,
    pub behavior: Box<dyn MonsterBehavior>,
}
