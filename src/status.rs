#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Vulnerable,
    Strength,
    Brutality,
    DemonForm,
    Weak,
    Dexterity,
    Frail,
    NoBlock,
    Thorns,
}

impl Status {
    pub fn decays(&self) -> bool {
        use Status::*;
        matches!(self, Vulnerable | Weak | NoBlock | Frail)
    }
}

#[cfg(test)]
mod tests {
    use super::Status::*;
    use crate::{
        actions::{block::BlockAction, damage::DamageAction, set_hp::SetHPAction},
        cards::CardClass,
        game::{CreatureRef, GameBuilder, Move},
        monsters::test::{ApplyVulnerableMonster, AttackMonster, NoopMonster},
    };

    #[test]
    fn test_strength() {
        let mut g = GameBuilder::default()
            .add_player_status(Strength, 2)
            .build_combat();

        let hp = g.monsters[0].creature.cur_hp;

        g.run_action(DamageAction::from_player(
            6,
            &g.player,
            &g.monsters[0].creature,
            CreatureRef::monster(0),
        ));

        assert_eq!(g.monsters[0].creature.cur_hp, hp - 8);
    }

    #[test]
    fn test_vulnerable() {
        let mut g = GameBuilder::default()
            .add_monster_status(Vulnerable, 2)
            .build_combat();

        assert_eq!(g.monsters[0].creature.statuses.get(&Vulnerable), Some(&2));

        let hp = g.monsters[0].creature.cur_hp;

        g.run_action(DamageAction::from_player(
            6,
            &g.player,
            &g.monsters[0].creature,
            CreatureRef::monster(0),
        ));

        assert_eq!(g.monsters[0].creature.cur_hp, hp - 9);

        g.make_move(Move::EndTurn);

        assert_eq!(g.monsters[0].creature.statuses.get(&Vulnerable), Some(&1));

        g.make_move(Move::EndTurn);

        assert_eq!(g.monsters[0].creature.statuses.get(&Vulnerable), None);
    }

    #[test]
    fn test_vulnerable2() {
        let mut g = GameBuilder::default()
            .add_monster(ApplyVulnerableMonster())
            .add_monster(NoopMonster())
            .add_card(CardClass::DebugKill)
            .build_combat();

        assert_eq!(g.player.creature.statuses.get(&Vulnerable), None);

        g.make_move(Move::EndTurn);

        assert_eq!(g.player.creature.statuses.get(&Vulnerable), Some(&2));

        g.make_move(Move::EndTurn);

        assert_eq!(g.player.creature.statuses.get(&Vulnerable), Some(&3));

        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        g.make_move(Move::EndTurn);

        assert_eq!(g.player.creature.statuses.get(&Vulnerable), Some(&2));

        g.make_move(Move::EndTurn);

        assert_eq!(g.player.creature.statuses.get(&Vulnerable), Some(&1));

        g.make_move(Move::EndTurn);

        assert_eq!(g.player.creature.statuses.get(&Vulnerable), None);
    }

    #[test]
    fn test_vulnerable3() {
        let mut g = GameBuilder::default()
            .add_monster(ApplyVulnerableMonster())
            .add_monster(ApplyVulnerableMonster())
            .build_combat();

        assert_eq!(g.player.creature.statuses.get(&Vulnerable), None);

        g.make_move(Move::EndTurn);

        assert_eq!(g.player.creature.statuses.get(&Vulnerable), Some(&4));
    }

    #[test]
    fn test_weak() {
        let mut g = GameBuilder::default()
            .add_player_status(Weak, 2)
            .build_combat();

        let hp = g.monsters[0].creature.cur_hp;

        g.run_action(DamageAction::from_player(
            6,
            &g.player,
            &g.monsters[0].creature,
            CreatureRef::monster(0),
        ));

        assert_eq!(g.monsters[0].creature.cur_hp, hp - 4);
    }

    #[test]
    fn test_multiple_damage_statuses() {
        let mut g = GameBuilder::default()
            .add_player_status(Weak, 2)
            .add_player_status(Strength, 4)
            .add_monster_status(Vulnerable, 2)
            .build_combat();

        let hp = g.monsters[0].creature.cur_hp;

        g.run_action(DamageAction::from_player(
            10,
            &g.player,
            &g.monsters[0].creature,
            CreatureRef::monster(0),
        ));

        assert_eq!(g.monsters[0].creature.cur_hp, hp - 15);
    }

    #[test]
    fn test_dexterity() {
        let mut g = GameBuilder::default()
            .add_player_status(Dexterity, 2)
            .build_combat();

        g.run_action(BlockAction {
            target: CreatureRef::player(),
            amount: 6,
        });

        assert_eq!(g.player.creature.block, 8);
    }

    #[test]
    fn test_frail() {
        let mut g = GameBuilder::default()
            .add_player_status(Frail, 2)
            .build_combat();

        g.run_action(BlockAction {
            target: CreatureRef::player(),
            amount: 6,
        });

        assert_eq!(g.player.creature.block, 4);
    }

    #[test]
    fn test_noblock() {
        let mut g = GameBuilder::default()
            .add_player_status(NoBlock, 2)
            .build_combat();

        g.run_action(BlockAction {
            target: CreatureRef::player(),
            amount: 6,
        });

        assert_eq!(g.player.creature.block, 0);
    }

    #[test]
    fn test_multiple_block_statuses() {
        let mut g = GameBuilder::default()
            .add_player_status(Dexterity, 4)
            .add_player_status(Frail, 2)
            .build_combat();

        g.run_action(BlockAction {
            target: CreatureRef::player(),
            amount: 6,
        });

        assert_eq!(g.player.creature.block, 7);
    }

    #[test]
    fn test_thorns() {
        let mut g = GameBuilder::default()
            .add_monster_status(Thorns, 2)
            .add_cards(CardClass::Strike, 5)
            .build_combat();

        let hp = g.player.creature.cur_hp;

        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });

        assert_eq!(g.player.creature.cur_hp, hp - 2);

        g.run_action(BlockAction {
            target: CreatureRef::player(),
            amount: 3,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.player.creature.block, 1);
        assert_eq!(g.player.creature.cur_hp, hp - 2);

        g.run_action(SetHPAction {
            target: CreatureRef::monster(0),
            hp: 3,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });

        assert_eq!(g.player.creature.block, 0);
        assert_eq!(g.player.creature.cur_hp, hp - 3);
    }

    #[test]
    fn test_thorns2() {
        let mut g = GameBuilder::default()
            .add_monster_status(Thorns, 2)
            .add_cards(CardClass::TwinStrike, 5)
            .build_combat();

        let hp = g.player.creature.cur_hp;

        g.run_action(SetHPAction {
            target: CreatureRef::monster(0),
            hp: 3,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });

        assert_eq!(g.player.creature.cur_hp, hp - 2);
    }

    #[test]
    fn test_thorns3() {
        let mut g = GameBuilder::default()
            .add_player_status(Thorns, 2)
            .add_monster(AttackMonster())
            .build_combat();

        g.run_action(SetHPAction {
            target: CreatureRef::monster(0),
            hp: 10,
        });
        g.run_action(BlockAction {
            target: CreatureRef::monster(0),
            amount: 5,
        });
        g.make_move(Move::EndTurn);

        assert_eq!(g.monsters[0].creature.cur_hp, 8);
    }
}
