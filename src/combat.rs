use rand::Rng;

use crate::{
    actions::{
        discard_card::DiscardCardAction, draw::DrawAction,
        end_of_turn_discard::EndOfTurnDiscardAction, play_card::PlayCardAction,
        start_of_turn_energy::StartOfTurnEnergyAction,
    },
    draw_pile::DrawPile,
    game::{CreatureRef, Game, RunActionsGameState, UsePotionStep},
    monster::{Monster, MonsterInfo},
    monsters::{
        blue_slaver::BlueSlaver, cultist::Cultist, fungi_beast::FungiBeast,
        gremlin_fat::GremlinFat, gremlin_mad::GremlinMad, gremlin_nob::GremlinNob,
        gremlin_shield::GremlinShield, gremlin_sneaky::GremlinSneaky,
        gremlin_wizard::GremlinWizard, jawworm::JawWorm, louse::Louse, red_slaver::RedSlaver,
        slime_acid_m::SlimeAcidM, slime_acid_s::SlimeAcidS, slime_spike_m::SlimeSpikeM,
        slime_spike_s::SlimeSpikeS, test::NoopMonster,
    },
    potion::random_potion_weighted,
    relic::RelicClass,
    rewards::{RewardType, Rewards, RewardsGameState},
    state::{GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct RollCombatGameState;

impl GameState for RollCombatGameState {
    fn run(&self, game: &mut Game) {
        if let Some(m) = game.force_monsters.take() {
            game.monsters = m;
        } else if game.roll_noop_monsters {
            game.monsters = vec![Monster::new(NoopMonster::new(), &mut game.rng)];
        } else {
            let m = match game.rng.random_range(0..9) {
                0 => vec![Monster::new(JawWorm::new(), &mut game.rng)],
                1 => vec![Monster::new(Cultist::new(), &mut game.rng)],
                2 => vec![
                    Monster::new(Louse::green(&mut game.rng), &mut game.rng),
                    Monster::new(Louse::red(&mut game.rng), &mut game.rng),
                ],
                3 => vec![Monster::new(SlimeAcidM::new(), &mut game.rng)],
                4 => vec![Monster::new(SlimeSpikeM::new(), &mut game.rng)],
                5 => vec![Monster::new(FungiBeast::new(), &mut game.rng)],
                6 => vec![
                    Monster::new(SlimeSpikeS::new(), &mut game.rng),
                    Monster::new(SlimeAcidS::new(), &mut game.rng),
                ],
                7 => vec![Monster::new(RedSlaver::new(), &mut game.rng)],
                8 => vec![Monster::new(BlueSlaver::new(), &mut game.rng)],
                _ => vec![
                    Monster::new(GremlinFat::new(), &mut game.rng),
                    Monster::new(GremlinMad::new(), &mut game.rng),
                    Monster::new(GremlinShield::new(), &mut game.rng),
                    Monster::new(GremlinSneaky::new(), &mut game.rng),
                    Monster::new(GremlinWizard::new(), &mut game.rng),
                ],
            };
            game.monsters = m;
        }
        game.state
            .push_state(RollCombatRewardsGameState(RewardType::Monster));
        game.state.push_state(CombatBeginGameState);
    }
}

#[derive(Debug)]
pub struct RollEliteCombatGameState;

impl GameState for RollEliteCombatGameState {
    fn run(&self, game: &mut Game) {
        game.monsters = vec![Monster::new(GremlinNob::new(), &mut game.rng)];
        game.state
            .push_state(RollCombatRewardsGameState(RewardType::Elite));
        game.state.push_state(CombatBeginGameState);
    }
}

#[derive(Debug)]
struct PlayerTurnEndGameState;

impl GameState for PlayerTurnEndGameState {
    fn run(&self, game: &mut Game) {
        if game.all_monsters_dead() {
            game.state.push_state(CombatEndGameState);
            return;
        }
        game.should_add_extra_decay_status = true;
        game.trigger_relics_at_turn_end();
        game.player
            .trigger_statuses_turn_end(CreatureRef::player(), &mut game.action_queue);

        // trigger card end of turn effects
        let mut indexes_to_discard = Vec::new();
        let mut actions = vec![];
        for (i, c) in game.hand.iter().enumerate() {
            if let Some(a) = c.borrow().class.end_of_turn_in_hand_behavior() {
                indexes_to_discard.push(i);
                actions.push(a);
            }
        }
        for a in actions {
            a(game);
        }
        for i in indexes_to_discard.into_iter().rev() {
            game.action_queue
                .push_top(DiscardCardAction(game.hand.remove(i)));
        }

        game.action_queue.push_bot(EndOfTurnDiscardAction());
        game.state.push_state(MonsterTurnGameState);
        game.state.push_state(RunActionsGameState);
    }
}

#[derive(Debug)]
struct MonsterTurnGameState;

impl GameState for MonsterTurnGameState {
    fn run(&self, game: &mut Game) {
        if game.all_monsters_dead() {
            game.state.push_state(CombatEndGameState);
            return;
        }
        for i in 0..game.monsters.len() {
            if !game.monsters[i].creature.is_alive() {
                continue;
            }
            game.monsters[i].creature.start_of_turn_lose_block(false);
            game.monsters[i]
                .creature
                .trigger_statuses_turn_begin(CreatureRef::monster(i), &mut game.action_queue);
        }
        for m in game.get_alive_monsters() {
            game.monster_turn_queue.push(m);
        }

        game.state.push_state(EndOfRoundGameState);
        game.state.push_state(RunActionsGameState);
    }
}

#[derive(Debug)]
struct EndOfRoundGameState;

impl GameState for EndOfRoundGameState {
    fn run(&self, game: &mut Game) {
        if game.all_monsters_dead() {
            game.state.push_state(CombatEndGameState);
            return;
        }
        game.should_add_extra_decay_status = false;
        for i in 0..game.monsters.len() {
            if !game.monsters[i].creature.is_alive() {
                continue;
            }
            game.monsters[i]
                .creature
                .trigger_statuses_turn_end(CreatureRef::monster(i), &mut game.action_queue);
        }
        game.player
            .trigger_statuses_round_end(CreatureRef::player(), &mut game.action_queue);
        for (i, m) in game.monsters.iter_mut().enumerate() {
            m.creature
                .trigger_statuses_round_end(CreatureRef::monster(i), &mut game.action_queue);
        }
        game.state.push_state(PlayerTurnBeginGameState);
        game.state.push_state(RunActionsGameState);
        game.turn += 1;
    }
}

#[derive(Debug)]
struct CombatEndGameState;

impl GameState for CombatEndGameState {
    fn run(&self, game: &mut Game) {
        game.state.push_state(ResetCombatGameState);

        game.trigger_relics_at_combat_finish();
        game.state.push_state(RunActionsGameState);
    }
}

#[derive(Debug)]
struct ResetCombatGameState;

impl GameState for ResetCombatGameState {
    fn run(&self, game: &mut Game) {
        game.monsters.clear();
        game.player.clear_all_status();
        game.num_cards_played_this_turn = 0;
        game.num_times_took_damage = 0;
        game.energy = 0;
        game.turn = 0;
        game.clear_all_piles();
    }
}

#[derive(Debug)]
struct RollCombatRewardsGameState(RewardType);

impl GameState for RollCombatRewardsGameState {
    fn run(&self, game: &mut Game) {
        // FIXME: no rewards when all monsters escape
        match self.0 {
            RewardType::Monster => {
                let gold = game.rng.random_range(10..=20);
                game.rewards.add_gold(gold);

                let cards = Rewards::gen_card_reward(game);
                game.rewards.add_cards(cards);
            }
            RewardType::Elite => {
                let gold = game.rng.random_range(25..=35);
                game.rewards.add_gold(gold);

                let cards = Rewards::gen_card_reward(game);
                game.rewards.add_cards(cards);

                let r = game.next_relic_weighted();
                game.rewards.add_relic(r);
            }
        }

        if game.rng.random_range(0..100) < game.potion_chance
            || game.has_relic(RelicClass::WhiteBeastStatue)
        {
            game.potion_chance -= 10;
            let p = random_potion_weighted(&mut game.rng);
            game.rewards.add_potion(p);
        } else {
            game.potion_chance += 10;
        }

        game.state.push_state(RewardsGameState);
    }
}

fn setup_combat_draw_pile(game: &mut Game) {
    let mut non_innate = Vec::new();
    let mut innate = Vec::new();
    for c in &game.master_deck {
        let c = game.clone_card_ref_same_id(c);
        if c.borrow().is_innate() {
            innate.push(c);
        } else {
            non_innate.push(c);
        }
    }
    let num_innate = innate.len() as i32;
    game.draw_pile = DrawPile::new(innate, non_innate);
    let extra_draw = num_innate - game.draw_per_turn;
    if extra_draw > 0 {
        game.action_queue.push_bot(DrawAction(extra_draw));
    }
}

#[derive(Debug)]
pub struct CombatBeginGameState;

impl GameState for CombatBeginGameState {
    fn run(&self, game: &mut Game) {
        game.turn = 0;
        game.should_add_extra_decay_status = false;

        setup_combat_draw_pile(game);

        // player pre-combat relic setup
        game.trigger_relics_at_pre_combat();

        // monster pre-combat setup
        for i in 0..game.monsters.len() {
            if !game.monsters[i].creature.is_alive() {
                continue;
            }
            game.monsters[i].behavior.pre_combat(
                &mut game.action_queue,
                CreatureRef::monster(i),
                &mut game.rng,
            );
        }

        game.state.push_state(PlayerTurnBeginGameState);
        game.state.push_state(RunActionsGameState);
    }
}

fn calculate_monster_info(game: &Game) -> MonsterInfo {
    MonsterInfo {
        num_monsters: game.monsters.len(),
        num_alive_monsters: game.get_alive_monsters().len(),
    }
}

#[derive(Debug)]
struct PlayerTurnBeginGameState;

impl GameState for PlayerTurnBeginGameState {
    fn run(&self, game: &mut Game) {
        if game.all_monsters_dead() {
            game.state.push_state(CombatEndGameState);
            return;
        }

        let info = calculate_monster_info(game);
        for i in 0..game.monsters.len() {
            if !game.monsters[i].creature.is_alive() {
                continue;
            }
            game.monsters[i]
                .behavior
                .roll_next_action(&mut game.rng, &info);
        }

        game.num_cards_played_this_turn = 0;

        game.player
            .start_of_turn_lose_block(game.has_relic(RelicClass::Calipers));

        if game.turn == 0 {
            game.trigger_relics_at_combat_begin_pre_draw();
        }
        game.trigger_relics_at_turn_begin_pre_draw();
        game.player
            .trigger_statuses_turn_begin(CreatureRef::player(), &mut game.action_queue);

        game.action_queue.push_bot(DrawAction(game.draw_per_turn));

        if game.turn == 0 {
            game.trigger_relics_at_combat_begin_post_draw();
        }

        game.trigger_relics_at_turn_begin_post_draw();
        game.player
            .trigger_statuses_turn_begin_post_draw(CreatureRef::player(), &mut game.action_queue);

        game.action_queue.push_top(StartOfTurnEnergyAction());

        game.state.push_state(PlayerTurnGameState);
        game.state.push_state(RunActionsGameState);
    }
}

#[derive(Debug)]
struct PlayerTurnGameState;

impl GameState for PlayerTurnGameState {
    fn run(&self, game: &mut Game) {
        if game.all_monsters_dead() {
            game.state.push_state(CombatEndGameState);
        }
    }
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        if game.all_monsters_dead() {
            return None;
        }
        let mut moves = Steps::default();
        moves.push(EndTurnStep);
        for (ci, c) in game.hand.iter().enumerate() {
            if !game.can_play_card(&PlayCardAction::new(c.clone(), None, game)) {
                continue;
            }
            let c = c.borrow();
            if c.has_target() {
                for (mi, m) in game.monsters.iter().enumerate() {
                    if !m.creature.is_alive() {
                        continue;
                    }
                    moves.push(PlayCardStep {
                        hand_index: ci,
                        target: Some(mi),
                    });
                }
            } else {
                moves.push(PlayCardStep {
                    hand_index: ci,
                    target: None,
                });
            }
        }
        for (pi, p) in game.potions.iter().enumerate() {
            if let Some(p) = p
                && p.can_use()
                && !p.can_use_outside_combat()
            {
                if p.has_target() {
                    for (mi, m) in game.monsters.iter().enumerate() {
                        if !m.creature.is_alive() {
                            continue;
                        }
                        moves.push(UsePotionStep {
                            potion_index: pi,
                            target: Some(mi),
                        });
                    }
                } else {
                    moves.push(UsePotionStep {
                        potion_index: pi,
                        target: None,
                    });
                }
            }
        }
        Some(moves)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct EndTurnStep;

impl Step for EndTurnStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        game.state.push_state(PlayerTurnEndGameState);
    }

    fn description(&self, _: &Game) -> String {
        "end turn".to_string()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct PlayCardStep {
    pub hand_index: usize,
    pub target: Option<usize>,
}

impl Step for PlayCardStep {
    fn should_pop_state(&self) -> bool {
        false
    }

    fn run(&self, game: &mut Game) {
        let c = game.hand.remove(self.hand_index);
        let action = PlayCardAction::new(c, self.target.map(CreatureRef::monster), game);
        assert!(game.can_play_card(&action));
        game.card_queue.push(action);
        game.state.push_state(RunActionsGameState);
    }

    fn description(&self, game: &Game) -> String {
        let mut s = format!(
            "play card {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        );
        if let Some(t) = self.target {
            s += &format!(
                " on monster {} ({})",
                t,
                game.monster_str(CreatureRef::monster(t))
            );
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::block::BlockAction,
        assert_matches,
        cards::{CardClass, CardCost},
        game::{DiscardPotionStep, GameBuilder, GameStatus},
        monsters::test::{AttackMonster, NoopMonster},
        potion::Potion,
        status::Status,
    };

    use super::*;

    #[test]
    fn test_moves() {
        let mut g = GameBuilder::default()
            .build_combat_with_monsters(NoopMonster::new(), NoopMonster::new());
        g.add_card_to_hand(CardClass::DebugKill);
        g.add_card_to_hand(CardClass::Defend);
        g.add_potion(Potion::Fire);
        g.add_potion(Potion::Flex);
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(EndTurnStep) as Box<dyn Step>,
                Box::new(PlayCardStep {
                    hand_index: 0,
                    target: Some(0),
                }),
                Box::new(PlayCardStep {
                    hand_index: 0,
                    target: Some(1),
                }),
                Box::new(PlayCardStep {
                    hand_index: 1,
                    target: None,
                }),
                Box::new(UsePotionStep {
                    potion_index: 0,
                    target: Some(0),
                }),
                Box::new(UsePotionStep {
                    potion_index: 0,
                    target: Some(1),
                }),
                Box::new(UsePotionStep {
                    potion_index: 1,
                    target: None,
                }),
                Box::new(DiscardPotionStep { potion_index: 0 }),
                Box::new(DiscardPotionStep { potion_index: 1 }),
            ]
        );
    }

    #[test]
    fn test_player_lose_block_start_of_turn() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(BlockAction::player_flat_amount(7));
        assert_eq!(g.player.block, 7);
        g.step_test(EndTurnStep);
        assert_eq!(g.player.block, 0);
    }

    #[test]
    fn test_monster_lose_block_start_of_turn() {
        let mut g = GameBuilder::default().build_combat();
        g.run_action(BlockAction::monster(CreatureRef::monster(0), 7));
        assert_eq!(g.monsters[0].creature.block, 7);
        g.step_test(EndTurnStep);
        assert_eq!(g.monsters[0].creature.block, 0);
    }

    #[test]
    fn test_barricade() {
        let mut g = GameBuilder::default()
            .add_monster_status(Status::Barricade, 1)
            .add_player_status(Status::Barricade, 1)
            .build_combat();
        g.run_action(BlockAction::monster(CreatureRef::player(), 7));
        g.run_action(BlockAction::monster(CreatureRef::monster(0), 7));
        g.step_test(EndTurnStep);
        assert_eq!(g.player.block, 7);
        assert_eq!(g.monsters[0].creature.block, 7);
    }

    #[test]
    fn test_unplayable_card_in_card_queue() {
        let mut g = GameBuilder::default()
            .add_player_status(Status::Entangled, 1)
            .build_combat();
        let c = g.new_card(CardClass::Thunderclap);
        g.card_queue.push(PlayCardAction::new(c, None, &g));
        g.run_all_actions();
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.energy, 3);
    }

    #[test]
    fn test_free_to_play() {
        let mut g = GameBuilder::default().build_combat();
        let c = g.new_card(CardClass::Defend);
        match &mut c.borrow_mut().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => *free_to_play_once = true,
            _ => panic!(),
        }
        g.hand.push(c);
        assert_eq!(g.energy, 3);
        g.step_test(PlayCardStep {
            hand_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
        g.hand.push(g.discard_pile.pop().unwrap());
        g.step_test(PlayCardStep {
            hand_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 2);

        g.step_test(EndTurnStep);
        assert_eq!(g.energy, 3);
        match &mut g.hand[0].borrow_mut().cost {
            CardCost::Cost {
                free_to_play_once, ..
            } => *free_to_play_once = true,
            _ => panic!(),
        }
        g.step_test(EndTurnStep);
        g.step_test(PlayCardStep {
            hand_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
    }

    #[test]
    fn test_has_relic() {
        use RelicClass::{BagOfPrep, BloodVial};
        let mut g = GameBuilder::default().build();

        assert!(!g.has_relic(BagOfPrep));
        assert!(!g.has_relic(BloodVial));

        g.add_relic(BagOfPrep);
        assert!(g.has_relic(BagOfPrep));
        assert!(!g.has_relic(BloodVial));

        g.remove_relic(BagOfPrep);
        assert!(!g.has_relic(BagOfPrep));
        assert!(!g.has_relic(BloodVial));
    }

    #[test]
    fn test_potions() {
        use Potion::{Attack, Skill};
        let mut g = GameBuilder::default().build();
        assert_eq!(g.potions, vec![None, None]);

        g.add_potion(Attack);
        assert_eq!(g.potions, vec![Some(Attack), None]);

        g.add_potion(Skill);
        assert_eq!(g.potions, vec![Some(Attack), Some(Skill)]);

        assert_eq!(g.take_potion(0), Attack);
        assert_eq!(g.potions, vec![None, Some(Skill)]);

        g.add_potion(Attack);
        assert_eq!(g.potions, vec![Some(Attack), Some(Skill)]);

        assert_eq!(g.take_potion(1), Skill);
        assert_eq!(g.potions, vec![Some(Attack), None]);
    }

    #[test]
    fn test_multi_attack_die_to_thorns() {
        let mut g = GameBuilder::default()
            .add_player_status(Status::Thorns, 999)
            .build_combat_with_monster(AttackMonster::with_attack_count(10, 10));
        g.player.cur_hp = 50;
        g.step_test(EndTurnStep);
        assert_eq!(g.player.cur_hp, 40);
    }

    #[test]
    fn test_card_queue_after_monsters_dead() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_draw_pile(CardClass::BandageUp);
        g.add_card_to_draw_pile(CardClass::DebugKill);
        g.add_card_to_draw_pile(CardClass::BandageUp);
        let hp = g.player.cur_hp;
        g.throw_potion(Potion::Chaos, None);
        assert_eq!(g.player.cur_hp, hp + 4);
    }

    #[test]
    fn test_defeat() {
        let mut g = GameBuilder::default().build_combat_with_monster(AttackMonster::new(999));
        g.step_test(EndTurnStep);
        assert_matches!(g.status, GameStatus::Defeat);
    }

    #[test]
    fn test_card_queue_duplicated_not_played() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::Normality);
        g.throw_potion(Potion::Duplication, None);
        g.throw_potion(Potion::Duplication, None);
        g.play_card(CardClass::Thunderclap, None);
        g.play_card(CardClass::Thunderclap, None);
        assert_eq!(g.discard_pile.len(), 2);
    }
}
