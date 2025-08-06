#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StatusType {
    Debuff,
    Buff,
    Amount,
}

macro_rules! s {
    ($($name:ident => $rarity:expr),+,) => {
        #[allow(dead_code)]
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
        pub enum Status {
            $(
                $name,
            )+
        }
        impl Status {
            #[allow(dead_code)]
            pub fn ty(&self) -> StatusType {
                use StatusType::*;
                match self {
                    $(Self::$name => $rarity,)+
                }
            }
        }
    };
}

s!(
    Artifact => Buff,
    Vulnerable => Debuff,
    Strength => Amount,
    Brutality => Buff,
    DemonForm => Buff,
    Weak => Debuff,
    Dexterity => Amount,
    Frail => Debuff,
    NoBlock => Debuff,
    Thorns => Buff,
    FeelNoPain => Buff,
    DarkEmbrace => Buff,
    Evolve => Buff,
    FireBreathing => Buff,
    Confusion => Debuff,
    Rupture => Buff,
    Barricade => Buff,
    NoDraw => Debuff,
    Duplication => Buff,
    DoubleTap => Buff,
    Bomb3 => Buff,
    Bomb2 => Buff,
    Bomb1 => Buff,
    Panache5 => Buff,
    Panache4 => Buff,
    Panache3 => Buff,
    Panache2 => Buff,
    Panache1 => Buff,
);

impl Status {
    pub fn decays(&self) -> bool {
        use Status::*;
        matches!(self, Vulnerable | Weak | NoBlock | Frail | Duplication)
    }
    pub fn disappears_end_of_turn(&self) -> bool {
        use Status::*;
        matches!(self, NoDraw | DoubleTap)
    }
    pub fn does_not_stack(&self) -> bool {
        use Status::*;
        matches!(self, NoDraw | Confusion | Barricade)
    }
    pub fn is_debuff(&self, amount: i32) -> bool {
        match self.ty() {
            StatusType::Amount => amount < 0,
            StatusType::Debuff => true,
            StatusType::Buff => false,
        }
    }
    pub fn bounded_999(&self) -> bool {
        use Status::*;
        // plated armor
        // gain strength
        matches!(self, Strength | Dexterity)
    }
}

#[cfg(test)]
mod tests {
    use super::Status::*;
    use crate::{
        actions::{
            block::BlockAction, damage::DamageAction, damage_all_monsters::DamageAllMonstersAction,
            draw::DrawAction, gain_status::GainStatusAction, reduce_status::ReduceStatusAction,
            set_hp::SetHPAction,
        },
        cards::{CardClass, CardCost, new_card},
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

    #[test]
    fn test_rupture() {
        let mut g = GameBuilder::default()
            .add_player_status(Rupture, 1)
            .add_monster(AttackMonster::new(1))
            .add_monster_status(Thorns, 1)
            .build_combat();

        g.play_card(CardClass::Strike, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.statuses.get(&Strength), None);

        g.play_card(CardClass::Bloodletting, None);
        assert_eq!(g.player.creature.statuses.get(&Strength), Some(&1));

        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.statuses.get(&Strength), Some(&1));

        g.hand.clear();
        g.hand.push(new_card(CardClass::Burn));
        g.run_action(BlockAction::player_flat_amount(2));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.statuses.get(&Strength), Some(&1));

        g.hand.clear();
        g.hand.push(new_card(CardClass::Burn));
        g.run_action(BlockAction::player_flat_amount(1));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.statuses.get(&Strength), Some(&2));

        g.hand.clear();
        g.hand.push(new_card(CardClass::Regret));
        g.run_action(BlockAction::player_flat_amount(2));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.statuses.get(&Strength), Some(&3));
    }

    #[test]
    fn test_stack() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(GainStatusAction {
            status: Confusion,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Confusion), Some(&1));
        g.run_action(GainStatusAction {
            status: Confusion,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Confusion), Some(&1));

        g.run_action(GainStatusAction {
            status: NoDraw,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&NoDraw), Some(&1));
        g.run_action(GainStatusAction {
            status: NoDraw,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&NoDraw), Some(&1));
    }

    #[test]
    fn test_reduce() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(GainStatusAction {
            status: Confusion,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Confusion), Some(&1));
        g.run_action(ReduceStatusAction {
            status: Confusion,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Confusion), None);

        g.run_action(GainStatusAction {
            status: Rupture,
            amount: 4,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Rupture), Some(&4));
        g.run_action(ReduceStatusAction {
            status: Rupture,
            amount: 3,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Rupture), Some(&1));
    }

    #[test]
    #[should_panic]
    fn test_reduce_amount_invalid() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(ReduceStatusAction {
            status: Strength,
            amount: 1,
            target: CreatureRef::player(),
        });
    }

    #[test]
    #[should_panic]
    fn test_reduce_negative_invalid() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(ReduceStatusAction {
            status: Rupture,
            amount: -1,
            target: CreatureRef::player(),
        });
    }

    #[test]
    #[should_panic]
    fn test_gain_non_stackable_invalid() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(GainStatusAction {
            status: Confusion,
            amount: 2,
            target: CreatureRef::player(),
        });
    }

    #[test]
    #[should_panic]
    fn test_gain_buff_invalid() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(GainStatusAction {
            status: Rupture,
            amount: -1,
            target: CreatureRef::player(),
        });
    }

    #[test]
    #[should_panic]
    fn test_gain_buff_invalid_2() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(GainStatusAction {
            status: Rupture,
            amount: 0,
            target: CreatureRef::player(),
        });
    }

    #[test]
    #[should_panic]
    fn test_gain_debuff_invalid() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(GainStatusAction {
            status: Weak,
            amount: -2,
            target: CreatureRef::player(),
        });
    }

    #[test]
    fn test_artifact() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(GainStatusAction {
            status: Artifact,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Artifact), Some(&1));
        g.run_action(GainStatusAction {
            status: Weak,
            amount: 2,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Artifact), None);
        assert_eq!(g.player.creature.statuses.get(&Weak), None);

        g.run_action(GainStatusAction {
            status: Artifact,
            amount: 2,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Artifact), Some(&2));
        g.run_action(GainStatusAction {
            status: Strength,
            amount: 2,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Artifact), Some(&2));
        assert_eq!(g.player.creature.statuses.get(&Strength), Some(&2));
        g.run_action(GainStatusAction {
            status: Strength,
            amount: -2,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Artifact), Some(&1));
        assert_eq!(g.player.creature.statuses.get(&Strength), Some(&2));
    }

    #[test]
    fn test_artifact_no_draw() {
        // getting no draw while already having no draw doesn't use artifact
        let mut g = GameBuilder::default().build_combat();

        g.run_action(GainStatusAction {
            status: NoDraw,
            amount: 1,
            target: CreatureRef::player(),
        });
        g.run_action(GainStatusAction {
            status: Artifact,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Artifact), Some(&1));
        assert_eq!(g.player.creature.statuses.get(&NoDraw), Some(&1));
        g.run_action(GainStatusAction {
            status: NoDraw,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Artifact), Some(&1));
        assert_eq!(g.player.creature.statuses.get(&NoDraw), Some(&1));
    }

    #[test]
    fn test_artifact_confusion() {
        // getting confusion while already having confusion does use artifact
        let mut g = GameBuilder::default().build_combat();

        g.run_action(GainStatusAction {
            status: Confusion,
            amount: 1,
            target: CreatureRef::player(),
        });
        g.run_action(GainStatusAction {
            status: Artifact,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Artifact), Some(&1));
        assert_eq!(g.player.creature.statuses.get(&Confusion), Some(&1));
        g.run_action(GainStatusAction {
            status: Confusion,
            amount: 1,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Artifact), None);
        assert_eq!(g.player.creature.statuses.get(&Confusion), Some(&1));
    }

    #[test]
    fn test_999() {
        let mut g = GameBuilder::default().build_combat();
        g.run_action(GainStatusAction {
            status: Strength,
            amount: 1000,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Strength), Some(&999));
        g.run_action(GainStatusAction {
            status: Strength,
            amount: -2000,
            target: CreatureRef::player(),
        });
        assert_eq!(g.player.creature.statuses.get(&Strength), Some(&-999));
    }

    #[test]
    fn test_duplication() {
        let mut g = GameBuilder::default()
            .add_player_status(Status::Duplication, 1)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::Rampage, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 8 - 8 - 5);
        assert_eq!(g.discard_pile.len(), 1);
        g.play_card(CardClass::Strike, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.statuses.get(&Duplication), None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 8 - 8 - 5 - 6);
    }

    #[test]
    fn test_duplication_2() {
        let mut g = GameBuilder::default()
            .add_player_status(Status::Duplication, 1)
            .build_combat();
        g.play_card(CardClass::PommelStrike, Some(CreatureRef::monster(0)));
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.hand.len(), 1);
    }

    #[test]
    fn test_duplication_duplicated_card() {
        let mut g = GameBuilder::default()
            .add_player_status(Status::Duplication, 2)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::Strike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6 - 6);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.player.creature.statuses.get(&Duplication), Some(&1));
    }

    #[test]
    fn test_double_tap() {
        let mut g = GameBuilder::default()
            .add_player_status(Status::DoubleTap, 1)
            .build_combat();
        g.energy = 99;

        let hp = g.monsters[0].creature.cur_hp;

        g.play_card(CardClass::Defend, None);
        assert_eq!(g.player.creature.block, 5);
        assert_eq!(g.player.creature.statuses.get(&DoubleTap), Some(&1));
        assert_eq!(g.discard_pile.len(), 1);

        g.play_card(CardClass::Rampage, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 8 - 8 - 5);
        assert_eq!(g.discard_pile.len(), 2);

        g.play_card(CardClass::Strike, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.statuses.get(&Duplication), None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 8 - 8 - 5 - 6);
    }

    #[test]
    fn test_duplication_double_tap() {
        let mut g = GameBuilder::default()
            .add_player_status(Status::Duplication, 1)
            .add_player_status(Status::DoubleTap, 1)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::Rampage, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 8 - 8 - 5 - 8 - 10);
        assert_eq!(g.discard_pile.len(), 1);
    }
}
