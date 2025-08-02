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
    FeelNoPain,
    DarkEmbrace,
    Evolve,
    FireBreathing,
    Confusion,
    Bomb3,
    Bomb2,
    Bomb1,
    Panache5,
    Panache4,
    Panache3,
    Panache2,
    Panache1,
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
        actions::{
            block::BlockAction, damage::DamageAction, damage_all_monsters::DamageAllMonstersAction,
            draw::DrawAction, set_hp::SetHPAction,
        },
        cards::{CardClass, CardCost},
        game::{CreatureRef, GameBuilder, Move},
        monsters::test::{ApplyStatusMonster, AttackMonster, NoopMonster},
        status::Status,
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
            .add_monster(ApplyStatusMonster {
                status: Status::Vulnerable,
                amount: 2,
            })
            .add_monster(NoopMonster::new())
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
            .add_monster(ApplyStatusMonster {
                status: Status::Vulnerable,
                amount: 2,
            })
            .add_monster(ApplyStatusMonster {
                status: Status::Vulnerable,
                amount: 2,
            })
            .build_combat();

        assert_eq!(g.player.creature.statuses.get(&Vulnerable), None);

        g.make_move(Move::EndTurn);

        assert_eq!(g.player.creature.statuses.get(&Vulnerable), Some(&4));
    }

    #[test]
    fn test_vulnerable4() {
        let mut g = GameBuilder::default()
            .add_monster_status(Vulnerable, 2)
            .build_combat();

        let hp = g.monsters[0].creature.cur_hp;

        g.run_action(DamageAllMonstersAction::from_player(6));

        assert_eq!(g.monsters[0].creature.cur_hp, hp - 9);
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

        g.run_action(BlockAction::player_card(6));

        assert_eq!(g.player.creature.block, 8);
    }

    #[test]
    fn test_frail() {
        let mut g = GameBuilder::default()
            .add_player_status(Frail, 2)
            .build_combat();

        g.run_action(BlockAction::player_card(6));

        assert_eq!(g.player.creature.block, 4);
    }

    #[test]
    fn test_noblock() {
        let mut g = GameBuilder::default()
            .add_player_status(NoBlock, 2)
            .build_combat();

        g.run_action(BlockAction::player_card(6));

        assert_eq!(g.player.creature.block, 0);
    }

    #[test]
    fn test_multiple_block_statuses() {
        let mut g = GameBuilder::default()
            .add_player_status(Dexterity, 4)
            .add_player_status(Frail, 2)
            .build_combat();

        g.run_action(BlockAction::player_card(6));

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

        g.run_action(BlockAction::player_flat_amount(3));
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
            .add_monster(AttackMonster::new(2))
            .build_combat();

        g.run_action(SetHPAction {
            target: CreatureRef::monster(0),
            hp: 10,
        });
        g.run_action(BlockAction::player_flat_amount(5));
        g.make_move(Move::EndTurn);

        assert_eq!(g.monsters[0].creature.cur_hp, 8);
    }

    #[test]
    fn test_confusion() {
        let mut found_0 = false;
        let mut found_1 = false;
        let mut found_2 = false;
        let mut found_3 = false;
        for _ in 0..100 {
            let g = GameBuilder::default()
                .add_player_status(Confusion, 1)
                .add_cards(CardClass::Strike, 10)
                .build_combat();
            for c in &g.hand {
                let c = c.borrow();
                match c.cost {
                    CardCost::Cost {
                        base_cost,
                        temporary_cost: _,
                    } => match base_cost {
                        0 => found_0 = true,
                        1 => found_1 = true,
                        2 => found_2 = true,
                        3 => found_3 = true,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
        }
        assert!(found_0);
        assert!(found_1);
        assert!(found_2);
        assert!(found_3);
    }

    #[test]
    fn test_confusion_temp_cost() {
        let mut g = GameBuilder::default()
            .add_player_status(Confusion, 1)
            .add_cards(CardClass::Strike, 10)
            .build_combat();
        for c in &g.draw_pile {
            let mut c = c.borrow_mut();
            match &mut c.cost {
                CardCost::Cost {
                    base_cost: _,
                    temporary_cost,
                } => {
                    *temporary_cost = Some(0);
                }
                _ => unreachable!(),
            }
        }
        g.run_action(DrawAction(2));
        for c in &g.hand {
            let c = c.borrow();
            match c.cost {
                CardCost::Cost {
                    base_cost: _,
                    temporary_cost,
                } => {
                    assert!(temporary_cost.is_none());
                }
                _ => unreachable!(),
            }
        }
    }
}
