use crate::{
    card::{Card, CardPile},
    creature::Creature,
    queue::ActionQueue,
    relic::{Relic, RelicClass, new_relic},
};

pub struct Player {
    pub creature: Creature,
    pub master_deck: CardPile,
    pub relics: Vec<Relic>,
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
        pub fn $func_name(&mut self, queue: &mut ActionQueue, card: &Card) {
            for r in &mut self.relics {
                r.$name(queue, card);
            }
        }
    };
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
    trigger!(trigger_relics_pre_combat, pre_combat);
    trigger!(trigger_relics_combat_start_pre_draw, combat_start_pre_draw);
    trigger!(
        trigger_relics_combat_start_post_draw,
        combat_start_post_draw
    );
    trigger!(trigger_relics_turn_start, turn_start);
    trigger!(trigger_relics_turn_end, turn_end);
    trigger!(trigger_relics_combat_finish, combat_finish);
    trigger_card!(trigger_relics_on_card_played, on_card_played);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creature::Creature;

    #[test]
    fn test_has_relic() {
        use RelicClass::{BagOfPrep, BloodVial};
        let mut p = Player {
            creature: Creature::new("test", 10),
            master_deck: vec![],
            relics: vec![],
        };

        assert!(!p.has_relic(BagOfPrep));
        assert!(!p.has_relic(BloodVial));

        p.add_relic(BagOfPrep);
        assert!(p.has_relic(BagOfPrep));
        assert!(!p.has_relic(BloodVial));

        p.remove_relic(BagOfPrep);
        assert!(!p.has_relic(BagOfPrep));
        assert!(!p.has_relic(BloodVial));
    }
}
