use rand::Rng;

use crate::{
    cards::{CardRarity, CardType},
    game::Game,
    master_deck::RemoveChosenCardsGameState,
    rng::rand_slice,
    state::{ContinueStep, GameState, Steps},
    step::Step,
};

#[derive(Debug)]
pub struct WeMeetAgainGameState {
    potion_index: Option<usize>,
    master_deck_index: Option<usize>,
    gold: Option<i32>,
}

impl WeMeetAgainGameState {
    pub fn new(game: &mut Game) -> Self {
        let potions = game
            .potions
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_some())
            .map(|e| e.0)
            .collect::<Vec<_>>();
        let potion_index = if !potions.is_empty() {
            Some(rand_slice(&mut game.rng, &potions))
        } else {
            None
        };

        let cards = game
            .master_deck
            .iter()
            .enumerate()
            .filter(|(_, c)| {
                c.borrow().class.rarity() != CardRarity::Basic
                    && c.borrow().class.ty() != CardType::Curse
            })
            .map(|e| e.0)
            .collect::<Vec<_>>();
        let master_deck_index = if !cards.is_empty() {
            Some(rand_slice(&mut game.rng, &cards))
        } else {
            None
        };

        let gold = if game.gold < 50 {
            None
        } else if game.gold > 150 {
            Some(game.rng.random_range(50..=150))
        } else {
            Some(game.rng.random_range(50..=game.gold))
        };

        Self {
            potion_index,
            master_deck_index,
            gold,
        }
    }
}

impl GameState for WeMeetAgainGameState {
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        if let Some(i) = self.potion_index {
            steps.push(OfferPotionStep { potion_index: i });
        }
        if let Some(i) = self.master_deck_index {
            steps.push(OfferCardStep {
                master_deck_index: i,
            });
        }
        if let Some(i) = self.gold {
            steps.push(OfferGoldStep { gold: i });
        }
        steps.push(ContinueStep);
        Some(steps)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OfferPotionStep {
    potion_index: usize,
}

impl Step for OfferPotionStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.take_potion(self.potion_index);
    }
    fn description(&self, game: &Game) -> String {
        format!("offer {:?}", game.potions[self.potion_index].unwrap())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OfferCardStep {
    master_deck_index: usize,
}

impl Step for OfferCardStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.chosen_cards
            .push(game.master_deck.remove(self.master_deck_index));
        game.state.push_state(RemoveChosenCardsGameState);
    }
    fn description(&self, game: &Game) -> String {
        format!(
            "offer {:?}",
            game.master_deck[self.master_deck_index].borrow()
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OfferGoldStep {
    gold: i32,
}

impl Step for OfferGoldStep {
    fn should_pop_state(&self) -> bool {
        true
    }
    fn run(&self, game: &mut Game) {
        game.gold -= self.gold;
        assert!(game.gold >= 0);
    }
    fn description(&self, _: &Game) -> String {
        format!("offer {} gold", self.gold)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cards::CardClass,
        events::Event,
        game::{AscendStep, GameBuilder, UsePotionStep},
        map::RoomType,
        potion::Potion,
    };

    use super::*;

    #[test]
    fn test_none() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::AscendersBane, 1)
            .add_cards(CardClass::Bash, 1)
            .build_with_rooms(&[RoomType::Event]);
        g.override_event_queue.push(Event::WeMeetAgain);
        g.step_test(AscendStep::new(0, 0));
        assert_eq!(
            g.valid_steps(),
            vec![Box::new(ContinueStep) as Box<dyn Step>]
        );
    }

    #[test]
    fn test_card() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::AscendersBane, 1)
            .add_cards(CardClass::Cleave, 1)
            .build_with_rooms(&[RoomType::Event]);
        g.master_deck[1].borrow_mut().is_bottled = true;
        g.override_event_queue.push(Event::WeMeetAgain);
        g.step_test(AscendStep::new(0, 0));
        g.step_test(OfferCardStep {
            master_deck_index: 1,
        });
        assert_eq!(g.master_deck.len(), 1);
    }

    #[test]
    fn test_potion() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event, RoomType::Monster]);
        g.add_potion(Potion::Blood);
        g.override_event_queue.push(Event::WeMeetAgain);
        g.step_test(AscendStep::new(0, 0));
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(OfferPotionStep { potion_index: 0 }) as Box<dyn Step>,
                Box::new(ContinueStep) as Box<dyn Step>,
            ]
        );
        g.step(0);
        assert!(g.potions[0].is_none());
    }

    #[test]
    fn test_can_use_potion_after() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event, RoomType::Monster]);
        g.add_potion(Potion::Blood);
        g.override_event_queue.push(Event::WeMeetAgain);
        g.step_test(AscendStep::new(0, 0));
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(OfferPotionStep { potion_index: 0 }) as Box<dyn Step>,
                Box::new(ContinueStep) as Box<dyn Step>,
            ]
        );
        g.step(1);
        g.step_test(AscendStep::new(0, 1));
        assert!(g.valid_steps().contains(
            &(Box::new(UsePotionStep {
                potion_index: 0,
                target: None
            }) as Box<dyn Step>)
        ));
    }

    #[test]
    fn test_gold_1() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event, RoomType::Monster]);
        g.gold = 50;
        g.override_event_queue.push(Event::WeMeetAgain);
        g.step_test(AscendStep::new(0, 0));
        assert_eq!(
            g.valid_steps(),
            vec![
                Box::new(OfferGoldStep { gold: 50 }) as Box<dyn Step>,
                Box::new(ContinueStep) as Box<dyn Step>,
            ]
        );
        g.step(0);
        assert_eq!(g.gold, 0);
    }

    #[test]
    fn test_gold_2() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event, RoomType::Monster]);
        g.gold = 200;
        g.override_event_queue.push(Event::WeMeetAgain);
        g.step_test(AscendStep::new(0, 0));
        g.step(0);
        assert!(g.gold >= 50);
    }

    #[test]
    fn test_gold_3() {
        let mut g = GameBuilder::default().build_with_rooms(&[RoomType::Event, RoomType::Monster]);
        g.gold = 75;
        g.override_event_queue.push(Event::WeMeetAgain);
        g.step_test(AscendStep::new(0, 0));
        g.step(0);
        assert!(g.gold <= 25);
    }
}
