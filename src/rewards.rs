use crate::{
    actions::{
        add_card_to_master_deck::AddCardToMasterDeckAction, gain_gold::GainGoldAction,
        gain_potion::GainPotionAction, gain_relic::GainRelicAction,
    },
    card::CardRef,
    cards::{CardRarity, random_common_red, random_rare_red, random_uncommon_red},
    game::{Game, RunActionsGameState},
    potion::Potion,
    relic::RelicClass,
    state::{GameState, Steps},
    step::Step,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RewardType {
    Monster,
    Elite,
}

#[derive(Default)]
pub struct Rewards {
    gold: i32,
    stolen_gold: i32,
    potions: Vec<Potion>,
    cards: Vec<Vec<CardRef>>,
    relics: Vec<RelicClass>,
}

impl Rewards {
    pub fn gen_card_reward(game: &mut Game) -> Vec<CardRef> {
        let mut num = 3;
        if game.has_relic(RelicClass::BustedCrown) {
            num -= 2;
        }
        if game.has_relic(RelicClass::QuestionCard) {
            num += 1;
        }
        let mut cards = Vec::<CardRef>::new();
        for _ in 0..num {
            let rarity = game.roll_rarity();
            match rarity {
                CardRarity::Common => game.rare_card_chance += 1,
                CardRarity::Uncommon => {}
                // FIXME: elite room
                CardRarity::Rare => game.rare_card_chance = -2,
                CardRarity::Basic | CardRarity::Special | CardRarity::Curse => panic!(),
            };
            let mut class;
            loop {
                class = match rarity {
                    CardRarity::Common => random_common_red(&mut game.rng),
                    CardRarity::Uncommon => random_uncommon_red(&mut game.rng),
                    CardRarity::Rare => random_rare_red(&mut game.rng),
                    CardRarity::Basic | CardRarity::Special | CardRarity::Curse => panic!(),
                };
                if cards.iter().all(|c| c.borrow().class != class) {
                    break;
                }
            }
            cards.push(game.new_card(class));
        }
        cards
    }
}

impl Rewards {
    pub fn add_gold(&mut self, mut amount: i32, has_golden_idol: bool) {
        if has_golden_idol {
            amount += (amount as f32 * 0.25).round() as i32
        }
        self.gold += amount;
    }
    pub fn add_stolen_gold(&mut self, amount: i32) {
        self.stolen_gold += amount;
    }
    pub fn add_potion(&mut self, potion: Potion) {
        self.potions.push(potion);
    }
    pub fn add_cards(&mut self, cards: Vec<CardRef>) {
        self.cards.push(cards);
    }
    pub fn add_relic(&mut self, relic: RelicClass) {
        self.relics.push(relic);
    }
}

#[derive(Debug)]
pub struct RewardsGameState;

impl GameState for RewardsGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        if !game.has_relic(RelicClass::Ectoplasm) {
            if game.rewards.gold != 0 {
                steps.push(GoldRewardStep);
            }
            if game.rewards.stolen_gold != 0 {
                steps.push(StolenGoldRewardStep);
            }
        }
        if !game.has_relic(RelicClass::Sozu) {
            for i in 0..game.rewards.potions.len() {
                steps.push(PotionRewardStep { potion_index: i });
            }
        }
        for (i, cards) in game.rewards.cards.iter().enumerate() {
            for ci in 0..cards.len() {
                steps.push(CardRewardStep {
                    pack_index: i,
                    card_index: ci,
                });
            }
        }
        for i in 0..game.rewards.relics.len() {
            steps.push(RelicRewardStep { relic_index: i });
        }
        steps.push(RewardExitStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GoldRewardStep;

impl Step for GoldRewardStep {
    fn should_pop_state(&self) -> bool {
        false
    }
    fn run(&self, game: &mut Game) {
        game.action_queue
            .push_bot(GainGoldAction(game.rewards.gold));
        game.rewards.gold = 0;
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, game: &Game) -> String {
        format!("gain {} gold", game.rewards.gold)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StolenGoldRewardStep;

impl Step for StolenGoldRewardStep {
    fn should_pop_state(&self) -> bool {
        false
    }
    fn run(&self, game: &mut Game) {
        game.action_queue
            .push_bot(GainGoldAction(game.rewards.stolen_gold));
        game.rewards.stolen_gold = 0;
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, game: &Game) -> String {
        format!("gain {} gold", game.rewards.stolen_gold)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PotionRewardStep {
    potion_index: usize,
}

impl Step for PotionRewardStep {
    fn should_pop_state(&self) -> bool {
        false
    }
    fn run(&self, game: &mut Game) {
        let p = game.rewards.potions.remove(self.potion_index);
        game.action_queue.push_bot(GainPotionAction(p));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, game: &Game) -> String {
        format!("gain {:?} potion", game.rewards.potions[self.potion_index])
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CardRewardStep {
    pack_index: usize,
    card_index: usize,
}

impl Step for CardRewardStep {
    fn should_pop_state(&self) -> bool {
        false
    }
    fn run(&self, game: &mut Game) {
        let mut pack = game.rewards.cards.remove(self.pack_index);
        let c = pack.remove(self.card_index);
        game.action_queue.push_bot(AddCardToMasterDeckAction(c));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, game: &Game) -> String {
        let all = game.rewards.cards[self.pack_index]
            .iter()
            .map(|c| format!("{:?}", c.borrow()))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "gain {:?} out of {}",
            game.rewards.cards[self.pack_index][self.card_index].borrow(),
            all
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RelicRewardStep {
    relic_index: usize,
}

impl Step for RelicRewardStep {
    fn should_pop_state(&self) -> bool {
        false
    }
    fn run(&self, game: &mut Game) {
        let r = game.rewards.relics.remove(self.relic_index);
        game.action_queue.push_bot(GainRelicAction(r));
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, game: &Game) -> String {
        format!("gain {:?}", game.rewards.relics[self.relic_index])
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct RewardExitStep;

impl Step for RewardExitStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.rewards = Rewards::default();
    }

    fn description(&self, _: &Game) -> String {
        "exit".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_matches,
        campfire::CampfireRestStep,
        cards::CardClass,
        combat::EndTurnStep,
        game::{AscendStep, GameBuilder},
        map::RoomType,
        monster::Intent,
        monsters::looter::Looter,
        relic::RelicRarity,
    };

    use super::*;

    #[test]
    fn test_combat_rewards() {
        let mut g = GameBuilder::default().build_combat();
        g.potion_chance = 0;
        g.play_card(CardClass::DebugKillAll, None);
        assert_ne!(g.rewards.gold, 0);
        assert_eq!(g.rewards.stolen_gold, 0);
        assert_eq!(g.rewards.cards.len(), 1);
        assert_eq!(g.rewards.cards[0].len(), 3);
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(GoldRewardStep) as Box<dyn Step>,
                Box::new(CardRewardStep {
                    pack_index: 0,
                    card_index: 0
                }),
                Box::new(CardRewardStep {
                    pack_index: 0,
                    card_index: 1
                }),
                Box::new(CardRewardStep {
                    pack_index: 0,
                    card_index: 2
                }),
                Box::new(RewardExitStep),
            ]
        );
        let gold = g.rewards.gold;
        g.step_test(GoldRewardStep);
        assert_eq!(g.gold, gold);
        assert!(gold >= 10);
        assert!(gold <= 20);
        let class = g.rewards.cards[0][0].borrow().class;
        g.step_test(CardRewardStep {
            pack_index: 0,
            card_index: 0,
        });
        assert_eq!(g.master_deck[0].borrow().class, class);
        assert_eq!(
            g.valid_steps(),
            vec![Box::new(RewardExitStep) as Box<dyn Step>]
        );
    }

    #[test]
    fn test_elite_rewards() {
        for _ in 0..10 {
            let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Elite]);
            g.potion_chance = 0;
            g.step_test(AscendStep { x: 0, y: 0 });
            g.play_card(CardClass::DebugKillAll, None);
            assert_ne!(g.rewards.gold, 0);
            assert_eq!(g.rewards.stolen_gold, 0);
            assert_eq!(g.rewards.cards.len(), 1);
            assert_eq!(g.rewards.cards[0].len(), 3);
            assert_eq!(g.rewards.relics.len(), 1);
            assert_eq!(
                g.valid_steps(),
                vec![
                    Box::new(GoldRewardStep) as Box<dyn Step>,
                    Box::new(CardRewardStep {
                        pack_index: 0,
                        card_index: 0
                    }),
                    Box::new(CardRewardStep {
                        pack_index: 0,
                        card_index: 1
                    }),
                    Box::new(CardRewardStep {
                        pack_index: 0,
                        card_index: 2
                    }),
                    Box::new(RelicRewardStep { relic_index: 0 }),
                    Box::new(RewardExitStep),
                ]
            );
            let gold = g.rewards.gold;
            g.step_test(GoldRewardStep);
            assert_eq!(g.gold, gold);
            assert!(gold >= 25);
            assert!(gold <= 35);
            g.step_test(RelicRewardStep { relic_index: 0 });
            assert_eq!(g.relics.len(), 1);
            assert_matches!(
                g.relics[0].get_class().rarity(),
                RelicRarity::Common | RelicRarity::Uncommon | RelicRarity::Rare
            );
        }
    }

    #[test]
    fn test_potion_reward() {
        let mut g = GameBuilder::default().build_combat();
        g.potion_chance = 100;
        g.play_card(CardClass::DebugKillAll, None);
        g.step_test(PotionRewardStep { potion_index: 0 });
        assert!(g.potions[0].is_some());
    }

    #[test]
    fn test_stolen_gold() {
        let mut g = GameBuilder::default().build_combat();
        g.rewards.stolen_gold = 100;
        g.play_card(CardClass::DebugKillAll, None);
        g.step_test(StolenGoldRewardStep);
        assert_eq!(g.gold, 100);
    }

    #[test]
    fn test_white_beast_statue() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::WhiteBeastStatue)
            .build_combat();
        g.play_card(CardClass::DebugKillAll, None);
        g.step_test(PotionRewardStep { potion_index: 0 });
        assert!(g.potions[0].is_some());
    }

    #[test]
    fn test_rare() {
        let mut found_rare = false;
        for _ in 0..1000 {
            let mut g =
                GameBuilder::default().build_with_rooms(&[RoomType::Monster, RoomType::Monster]);
            g.roll_noop_monsters = true;
            g.step_test(AscendStep { x: 0, y: 0 });
            g.play_card(CardClass::DebugKillAll, None);
            assert!(
                g.rewards
                    .cards
                    .iter()
                    .flatten()
                    .all(|c| c.borrow().class.rarity() != CardRarity::Rare)
            );
            g.step_test(RewardExitStep);
            g.step_test(AscendStep { x: 0, y: 1 });
            g.play_card(CardClass::DebugKillAll, None);
            found_rare = g
                .rewards
                .cards
                .iter()
                .flatten()
                .any(|c| c.borrow().class.rarity() == CardRarity::Rare);
            if found_rare {
                break;
            }
        }
        assert!(found_rare);
    }

    #[test]
    fn test_escape_rewards() {
        let mut g = GameBuilder::default().build_combat_with_monster(Looter::new());
        g.potion_chance = 100;
        loop {
            let done = matches!(g.monsters[0].behavior.get_intent(), Intent::Escape);
            g.step_test(EndTurnStep);
            if done {
                break;
            }
        }
        assert_eq!(g.rewards.gold, 0);
        assert!(!g.rewards.cards.is_empty());
        assert!(g.rewards.potions.is_empty());
    }

    #[test]
    fn test_smoke_bomb_rewards() {
        let mut g = GameBuilder::default().build_combat();
        g.throw_potion(Potion::Smoke, None);
        assert_eq!(
            g.valid_steps(),
            vec![Box::new(RewardExitStep) as Box<dyn Step>]
        );
    }

    #[test]
    fn test_campfire_dreamcatcher() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::DreamCatcher)
            .build_campfire();
        g.step_test(CampfireRestStep);
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(CardRewardStep {
                    pack_index: 0,
                    card_index: 0
                }) as Box<dyn Step>,
                Box::new(CardRewardStep {
                    pack_index: 0,
                    card_index: 1
                }),
                Box::new(CardRewardStep {
                    pack_index: 0,
                    card_index: 2
                }),
                Box::new(RewardExitStep)
            ]
        );
    }

    #[test]
    fn test_golden_idol() {
        for _ in 0..10 {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::GoldenIdol)
                .build_with_rooms(&[RoomType::Monster, RoomType::Elite]);
            g.step_test(AscendStep { x: 0, y: 0 });
            g.play_card(CardClass::DebugKillAll, None);
            assert!(g.rewards.gold >= 13);
            assert!(g.rewards.gold <= 25);
            g.step_test(RewardExitStep);

            g.step_test(AscendStep { x: 0, y: 1 });
            g.play_card(CardClass::DebugKillAll, None);
            assert!(g.rewards.gold >= 31);
            assert!(g.rewards.gold <= 44);
        }
        {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::GoldenIdol)
                .build_combat_with_monster(Looter::new());
            g.gold = 100;
            g.step_test(EndTurnStep);
            g.play_card(CardClass::DebugKillAll, None);
            assert_eq!(g.rewards.stolen_gold, 20);
        }
    }
}
