use crate::{
    actions::{
        gain_relic::GainRelicAction, heal::HealAction, increase_max_hp::IncreaseMaxHPAction,
        removed_card_from_master_deck::RemovedCardFromMasterDeckAction,
    },
    cards::CardRarity,
    game::{CreatureRef, Game, RunActionsGameState},
    relic::RelicClass,
    state::{GameState, NoopStep, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct BonfireGameState;

impl GameState for BonfireGameState {
    fn valid_steps(&self, game: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        for (i, c) in game.master_deck.iter().enumerate() {
            if c.borrow().can_remove_from_master_deck() {
                steps.push(OfferStep { master_index: i });
            }
        }
        if steps.steps.is_empty() {
            steps.push(NoopStep);
        }
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OfferStep {
    master_index: usize,
}

impl Step for OfferStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        let class = game.master_deck.remove(self.master_index).borrow().class;
        game.action_queue
            .push_bot(RemovedCardFromMasterDeckAction(class));
        match class.rarity() {
            CardRarity::Basic => {}
            CardRarity::Common | CardRarity::Special => game.action_queue.push_bot(HealAction {
                target: CreatureRef::player(),
                amount: 5,
            }),
            CardRarity::Uncommon => game.action_queue.push_bot(HealAction {
                target: CreatureRef::player(),
                amount: game.player.max_hp,
            }),
            CardRarity::Rare => {
                game.action_queue.push_bot(IncreaseMaxHPAction(10));
                game.action_queue.push_bot(HealAction {
                    target: CreatureRef::player(),
                    amount: game.player.max_hp,
                });
            }
            CardRarity::Curse => {
                game.action_queue
                    .push_bot(GainRelicAction(RelicClass::SpiritPoop));
            }
        }
        game.state.push_state(RunActionsGameState);
    }
    fn description(&self, game: &Game) -> String {
        format!("offer {:?}", game.master_deck[self.master_index].borrow())
    }
}

#[cfg(test)]
mod tests {
    use crate::{cards::CardClass, game::GameBuilder};

    use super::*;

    #[test]
    fn test_offer_common() {
        for c in [CardClass::Jax, CardClass::Anger] {
            let mut g = GameBuilder::default()
                .add_cards(c, 2)
                .build_with_game_state(BonfireGameState);
            g.player.max_hp = 50;
            g.player.cur_hp = 10;
            g.step_test(OfferStep { master_index: 0 });
            assert_eq!(g.player.max_hp, 50);
            assert_eq!(g.player.cur_hp, 15);
        }
    }

    #[test]
    fn test_offer_uncommon_special() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::FlameBarrier, 2)
            .build_with_game_state(BonfireGameState);
        g.player.max_hp = 50;
        g.player.cur_hp = 10;
        g.step_test(OfferStep { master_index: 0 });
        assert_eq!(g.player.max_hp, 50);
        assert_eq!(g.player.cur_hp, 50);
    }

    #[test]
    fn test_offer_rare() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Impervious, 2)
            .build_with_game_state(BonfireGameState);
        g.player.max_hp = 50;
        g.player.cur_hp = 10;
        g.step_test(OfferStep { master_index: 0 });
        assert_eq!(g.player.max_hp, 60);
        assert_eq!(g.player.cur_hp, 60);
    }

    #[test]
    fn test_offer_curse() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Pain, 2)
            .build_with_game_state(BonfireGameState);
        g.player.max_hp = 50;
        g.player.cur_hp = 10;
        g.step_test(OfferStep { master_index: 0 });
        assert_eq!(g.player.max_hp, 50);
        assert_eq!(g.player.cur_hp, 10);
        assert!(g.has_relic(RelicClass::SpiritPoop));
    }

    #[test]
    fn test_no_removable_cards() {
        let g = GameBuilder::default()
            .add_cards(CardClass::AscendersBane, 2)
            .build_with_game_state(BonfireGameState);
        assert_eq!(g.valid_steps(), vec![Box::new(NoopStep) as Box<dyn Step>]);
    }
}
