use crate::{
    action::Action,
    actions::block::BlockAction,
    game::{CreatureRef, Game},
    rng::rand_slice,
};

pub struct BlockRandomMonsterAction {
    pub source: CreatureRef,
    pub amount: i32,
}

impl Action for BlockRandomMonsterAction {
    fn run(&self, game: &mut Game) {
        let alive = game
            .get_alive_monsters()
            .into_iter()
            .filter(|t| *t != self.source)
            .collect::<Vec<_>>();
        let target = if alive.is_empty() {
            self.source
        } else {
            rand_slice(&mut game.rng, &alive)
        };
        game.action_queue
            .push_top(BlockAction::monster(target, self.amount));
    }
}

impl std::fmt::Debug for BlockRandomMonsterAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "block random monster {}", self.amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::block_random_monster::BlockRandomMonsterAction,
        cards::CardClass,
        game::{CreatureRef, GameBuilder},
        monsters::test::NoopMonster,
    };

    #[test]
    fn test_block_random_monster() {
        let mut g = GameBuilder::default()
            .build_combat_with_monsters(NoopMonster::new(), NoopMonster::new());
        g.run_action(BlockRandomMonsterAction {
            source: CreatureRef::monster(0),
            amount: 1,
        });
        assert_eq!(g.monsters[0].creature.block, 0);
        assert_eq!(g.monsters[1].creature.block, 1);
    }

    #[test]
    fn test_block_random_monster_dead() {
        let mut g = GameBuilder::default()
            .build_combat_with_monsters(NoopMonster::new(), NoopMonster::new());
        g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(1)));
        g.run_action(BlockRandomMonsterAction {
            source: CreatureRef::monster(0),
            amount: 1,
        });
        assert_eq!(g.monsters[0].creature.block, 1);
        assert_eq!(g.monsters[1].creature.block, 0);
    }
}
