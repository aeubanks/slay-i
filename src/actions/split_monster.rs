use crate::{
    action::Action,
    game::{CreatureRef, Game},
    monster::Monster,
    monsters::{slime_acid_m::SlimeAcidM, test::AttackMonster},
};

#[allow(dead_code)]
pub enum SplitMonsterType {
    TestAttack,
    SlimeAcidL,
}

pub struct SplitMonsterAction {
    pub monster: CreatureRef,
    pub ty: SplitMonsterType,
}

impl Action for SplitMonsterAction {
    fn run(&self, game: &mut Game) {
        let hp = game.get_creature(self.monster).cur_hp;
        game.get_creature_mut(self.monster).cur_hp = 0;
        let turn_pos = game
            .monster_turn_queue_all
            .iter()
            .position(|&c| c == self.monster)
            .unwrap();
        for _ in 0..2 {
            let m = match self.ty {
                SplitMonsterType::SlimeAcidL => Monster::new_with_hp(SlimeAcidM::new(), hp),
                SplitMonsterType::TestAttack => Monster::new_with_hp(AttackMonster::new(10), hp),
            };
            game.monsters.push(m);
            game.monster_turn_queue_all
                .insert(turn_pos, CreatureRef::monster(game.monsters.len() - 1));
        }
        game.monster_turn_queue_all.retain(|&c| c != self.monster);
    }
}

impl std::fmt::Debug for SplitMonsterAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "split {:?}", self.monster)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::damage::DamageAction,
        combat::EndTurnStep,
        game::{GameBuilder, Rand},
        monster::{Intent, MonsterBehavior, MonsterInfo},
        queue::ActionQueue,
    };

    struct TestSplitMonster;

    impl MonsterBehavior for TestSplitMonster {
        fn name(&self) -> &'static str {
            "test split monster"
        }

        fn hp_range(&self) -> (i32, i32) {
            (12, 12)
        }

        fn roll_next_action(&mut self, _: &mut Rand, _: &MonsterInfo) {}

        fn take_turn(&mut self, this: CreatureRef, queue: &mut ActionQueue, _: &MonsterInfo) {
            queue.push_bot(DamageAction::from_monster(2, this));
            queue.push_bot(SplitMonsterAction {
                monster: this,
                ty: SplitMonsterType::TestAttack,
            });
        }

        fn get_intent(&self) -> Intent {
            Intent::Unknown
        }
    }

    #[test]
    fn test_split() {
        let mut g = GameBuilder::default().build_combat_with_monster(TestSplitMonster);
        g.player.cur_hp = 50;
        g.step_test(EndTurnStep);
        assert_eq!(g.player.cur_hp, 48);
        g.set_debug();
        g.step_test(EndTurnStep);
        assert_eq!(g.monsters[0].creature.cur_hp, 0);
        assert_eq!(g.monsters[1].creature.cur_hp, 12);
        assert_eq!(g.monsters[2].creature.cur_hp, 12);
        assert_eq!(g.player.cur_hp, 28);
        g.step_test(EndTurnStep);
        assert_eq!(g.monsters.len(), 3);
        assert_eq!(g.player.cur_hp, 8);
    }
}
