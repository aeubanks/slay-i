use rand::Rng;

use crate::{
    actions::{damage::DamageAction, gain_status::GainStatusAction},
    game::{CreatureRef, Rand},
    monster::{Intent, MonsterBehavior, MonsterInfo},
    queue::ActionQueue,
    status::Status,
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Action {
    Start,
    Buff,
    Attack,
}

pub struct Cultist {
    action: Action,
}

impl Cultist {
    pub fn new() -> Self {
        Self {
            action: Action::Start,
        }
    }
}

impl MonsterBehavior for Cultist {
    fn name(&self) -> &'static str {
        "cultist"
    }
    fn roll_hp(&self, r: &mut Rand) -> i32 {
        r.random_range(50..=56)
    }

    fn take_turn(&mut self, this: CreatureRef, queue: &mut ActionQueue) {
        match self.action {
            Action::Start => panic!(),
            Action::Buff => queue.push_bot(GainStatusAction {
                status: Status::Ritual,
                amount: 5,
                target: this,
            }),
            Action::Attack => queue.push_bot(DamageAction::from_monster(1, this)),
        }
    }
    fn roll_next_action(&mut self, _: &mut Rand, _info: &MonsterInfo) {
        self.action = match self.action {
            Action::Start => Action::Buff,
            Action::Buff | Action::Attack => Action::Attack,
        };
    }

    fn get_intent(&self) -> Intent {
        match self.action {
            Action::Start => panic!(),
            Action::Buff => Intent::Buff,
            Action::Attack => Intent::Attack(1, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{combat::EndTurnStep, game::GameBuilder, monsters::cultist::Cultist};

    #[test]
    fn test_cultist() {
        let mut g = GameBuilder::default().build_combat_with_monster(Cultist::new());

        g.player.cur_hp = 50;
        g.step_test(EndTurnStep);
        assert_eq!(g.player.cur_hp, 50);
        g.step_test(EndTurnStep);
        assert_eq!(g.player.cur_hp, 50 - 6);

        g.player.cur_hp = 50;
        g.step_test(EndTurnStep);
        assert_eq!(g.player.cur_hp, 50 - 11);

        g.player.cur_hp = 50;
        g.step_test(EndTurnStep);
        assert_eq!(g.player.cur_hp, 50 - 16);
    }
}
