use crate::{
    action::Action,
    actions::heal::HealAction,
    game::{CreatureRef, Game},
};

pub struct HealPercentAction {
    pub target: CreatureRef,
    pub percent: i32,
}

impl Action for HealPercentAction {
    fn run(&self, game: &mut Game) {
        let amount = game.get_creature(self.target).max_hp as f32 * self.percent as f32 / 100.0;
        game.action_queue.push_top(HealAction {
            target: self.target,
            amount: amount as i32,
        });
    }
}

impl std::fmt::Debug for HealPercentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "heal {}% {:?}", self.percent, self.target)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::heal_percent::HealPercentAction,
        game::{CreatureRef, GameBuilder},
        monsters::test::NoopMonster,
    };

    #[test]
    fn test_heal_percent() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::with_hp(50))
            .build_combat();
        g.monsters[0].creature.cur_hp = 10;
        g.run_action(HealPercentAction {
            target: CreatureRef::monster(0),
            percent: 20,
        });
        assert_eq!(g.monsters[0].creature.cur_hp, 10 + (50.0 * 0.2) as i32);
    }
}
