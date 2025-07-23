use rand::Rng;

use crate::{
    actions::{damage::DamageAction, draw::DrawAction, heal::HealAction},
    card::Card,
    cards::CardType,
    game::{CreatureRef, Rand},
    queue::ActionQueue,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RelicClass {
    // Starter
    BurningBlood,
    // Common
    BagOfPrep,
    BloodVial,
    // Uncommon
    BlueCandle,
    // Shop
    MedicalKit,
}

type RelicCallback = fn(&mut i32, &mut ActionQueue);
type RelicCardCallback = fn(&mut i32, &mut ActionQueue, &Card);

impl RelicClass {
    pub fn pre_combat(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BloodVial => Some(blood_vial),
            _ => None,
        }
    }
    pub fn combat_finish(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BurningBlood => Some(burning_blood),
            _ => None,
        }
    }
    pub fn combat_start_pre_draw(&self) -> Option<RelicCallback> {
        None
    }
    pub fn combat_start_post_draw(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BagOfPrep => Some(bag_of_prep),
            _ => None,
        }
    }
    pub fn on_card_played(&self) -> Option<RelicCardCallback> {
        use RelicClass::*;
        match self {
            BlueCandle => Some(blue_candle),
            _ => None,
        }
    }
    pub fn turn_end(&self) -> Option<RelicCallback> {
        None
    }
}

fn burning_blood(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: 6,
    });
}

fn blood_vial(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: 2,
    });
}

fn bag_of_prep(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(DrawAction(2));
}

fn blue_candle(_: &mut i32, queue: &mut ActionQueue, card: &Card) {
    if card.class.ty() == CardType::Curse {
        queue.push_bot(DamageAction::lose_hp(1, CreatureRef::player()));
    }
}

pub struct Relic {
    class: RelicClass,
    value: i32,
}

impl Relic {
    pub fn get_class(&self) -> RelicClass {
        self.class
    }
}

macro_rules! trigger {
    ($name:ident) => {
        pub fn $name(&mut self, queue: &mut ActionQueue) {
            if let Some(f) = self.class.$name() {
                f(&mut self.value, queue)
            }
        }
    };
}
macro_rules! trigger_card {
    ($name:ident) => {
        pub fn $name(&mut self, queue: &mut ActionQueue, card: &Card) {
            if let Some(f) = self.class.$name() {
                f(&mut self.value, queue, card)
            }
        }
    };
}

impl Relic {
    trigger!(pre_combat);
    trigger!(combat_start_pre_draw);
    trigger!(combat_start_post_draw);
    trigger!(turn_end);
    trigger!(combat_finish);
    trigger_card!(on_card_played);
}

pub fn new_relic(class: RelicClass) -> Relic {
    Relic { class, value: 0 }
}

pub fn random_relic(rng: &mut Rand) -> RelicClass {
    use RelicClass::*;
    let relics = [BagOfPrep, BloodVial];
    let i = rng.random_range(0..relics.len());
    relics[i]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::block::BlockAction,
        cards::CardClass,
        game::{GameBuilder, Move},
    };

    #[test]
    fn test_burning_blood() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::DebugKill)
            .add_relic(RelicClass::BurningBlood)
            .build_combat();
        let hp = g.player.creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.player.creature.cur_hp, hp + 6);
    }

    #[test]
    fn test_blood_vial() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::BloodVial)
            .set_player_hp(50)
            .build_combat();
        assert_eq!(g.player.creature.cur_hp, 52);
    }

    #[test]
    fn test_bag_of_prep() {
        let g = GameBuilder::default()
            .ironclad_starting_deck()
            .add_relic(RelicClass::BagOfPrep)
            .build_combat();
        assert_eq!(g.hand.len(), 7);
    }

    #[test]
    fn test_medical_kit() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Wound)
            .add_card(CardClass::Dazed)
            .add_relic(RelicClass::MedicalKit)
            .set_player_hp(50)
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
        assert_eq!(g.player.creature.cur_hp, 50);
        assert_eq!(g.exhaust_pile.len(), 2);
        assert_eq!(g.discard_pile.len(), 0);
    }

    #[test]
    fn test_blue_candle() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::AscendersBane)
            .add_card(CardClass::Injury)
            .add_relic(RelicClass::BlueCandle)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction {
            target: CreatureRef::player(),
            amount: 5,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
        assert_eq!(g.player.creature.cur_hp, 48);
        assert_eq!(g.exhaust_pile.len(), 2);
        assert_eq!(g.discard_pile.len(), 0);
    }
}
