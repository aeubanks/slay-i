use crate::{
    actions::play_card::PlayCardAction,
    card::CardPile,
    creature::Creature,
    potion::Potion,
    queue::ActionQueue,
    relic::{Relic, RelicClass, new_relic},
};

pub struct Player {
    pub creature: Creature,
    pub master_deck: CardPile,
    pub relics: Vec<Relic>,
    pub potions: Vec<Option<Potion>>,
    pub gold: i32,
}

macro_rules! trigger {
    ($func_name:ident, $name:ident) => {
        pub fn $func_name(&mut self, queue: &mut ActionQueue) {
            for r in &mut self.relics {
                r.$name(queue);
            }
        }
    };
}

macro_rules! trigger_card {
    ($func_name:ident, $name:ident) => {
        pub fn $func_name(&mut self, queue: &mut ActionQueue, play: &PlayCardAction) {
            for r in &mut self.relics {
                r.$name(queue, play);
            }
        }
    };
}

impl Player {
    pub fn new(name: &'static str, max_hp: i32) -> Self {
        Self {
            creature: Creature::new(name, max_hp),
            master_deck: vec![],
            relics: vec![],
            potions: vec![None, None],
            gold: 0,
        }
    }
    #[cfg(test)]
    pub fn for_test() -> Self {
        Self::new("test", 100)
    }
}

impl Player {
    pub fn add_relic(&mut self, class: RelicClass) {
        self.relics.push(new_relic(class));
    }
    #[cfg(test)]
    pub fn remove_relic(&mut self, class: RelicClass) {
        self.relics.retain(|r| r.get_class() != class);
    }
    pub fn has_relic(&self, class: RelicClass) -> bool {
        self.relics.iter().any(|r| r.get_class() == class)
    }
    #[cfg(test)]
    pub fn get_relic_value(&self, class: RelicClass) -> Option<i32> {
        self.relics
            .iter()
            .find(|r| r.get_class() == class)
            .map(|r| r.get_value())
    }
    trigger!(trigger_relics_on_shuffle, on_shuffle);
    trigger!(trigger_relics_at_pre_combat, at_pre_combat);
    trigger!(
        trigger_relics_at_combat_start_pre_draw,
        at_combat_start_pre_draw
    );
    trigger!(
        trigger_relics_at_combat_start_post_draw,
        at_combat_start_post_draw
    );
    trigger!(trigger_relics_at_turn_start, at_turn_start);
    trigger!(trigger_relics_at_turn_end, at_turn_end);
    trigger!(trigger_relics_at_combat_finish, at_combat_finish);
    trigger_card!(trigger_relics_on_card_played, on_card_played);
}

impl Player {
    pub fn add_potion(&mut self, potion: Potion) {
        let mut added = false;
        for p in &mut self.potions {
            if p.is_none() {
                *p = Some(potion);
                added = true;
                break;
            }
        }
        assert!(added);
    }
    pub fn take_potion(&mut self, i: usize) -> Potion {
        let p = self.potions[i].unwrap();
        self.potions[i] = None;
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_relic() {
        use RelicClass::{BagOfPrep, BloodVial};
        let mut p = Player::for_test();

        assert!(!p.has_relic(BagOfPrep));
        assert!(!p.has_relic(BloodVial));

        p.add_relic(BagOfPrep);
        assert!(p.has_relic(BagOfPrep));
        assert!(!p.has_relic(BloodVial));

        p.remove_relic(BagOfPrep);
        assert!(!p.has_relic(BagOfPrep));
        assert!(!p.has_relic(BloodVial));
    }

    #[test]
    fn test_potions() {
        use Potion::{Attack, Skill};
        let mut p = Player::for_test();
        assert_eq!(p.potions, vec![None, None]);

        p.add_potion(Attack);
        assert_eq!(p.potions, vec![Some(Attack), None]);

        p.add_potion(Skill);
        assert_eq!(p.potions, vec![Some(Attack), Some(Skill)]);

        assert_eq!(p.take_potion(0), Attack);
        assert_eq!(p.potions, vec![None, Some(Skill)]);

        p.add_potion(Attack);
        assert_eq!(p.potions, vec![Some(Attack), Some(Skill)]);

        assert_eq!(p.take_potion(1), Skill);
        assert_eq!(p.potions, vec![Some(Attack), None]);
    }
}
