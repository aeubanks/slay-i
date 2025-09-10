use crate::{
    action::Action,
    game::{CreatureRef, Game},
};

pub struct HealAction {
    pub target: CreatureRef,
    pub amount: i32,
}

impl Action for HealAction {
    fn run(&self, game: &mut Game) {
        if game.get_creature(self.target).is_alive() {
            game.heal(self.target, self.amount);
        }
    }
}

impl std::fmt::Debug for HealAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "heal {} hp {:?}", self.amount, self.target)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{damage::DamageAction, heal::HealAction},
        game::{CreatureRef, GameBuilder},
        monsters::test::NoopMonster,
    };

    #[test]
    fn test_heal() {
        let mut g = GameBuilder::default()
            .add_monster(NoopMonster::with_hp(50))
            .add_monster(NoopMonster::with_hp(50))
            .build_combat();
        g.run_action(DamageAction::thorns_no_rupture(
            100,
            CreatureRef::monster(0),
        ));
        g.monsters[1].creature.cur_hp = 10;
        g.run_action(HealAction {
            target: CreatureRef::monster(0),
            amount: 5,
        });
        g.run_action(HealAction {
            target: CreatureRef::monster(1),
            amount: 5,
        });
        assert_eq!(g.monsters[0].creature.cur_hp, 0);
        assert!(!g.monsters[0].creature.is_alive());
        assert_eq!(g.monsters[1].creature.cur_hp, 15);
        assert!(g.monsters[1].creature.is_alive());
    }
}
