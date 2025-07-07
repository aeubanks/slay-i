use crate::{card::CardPile, creature::Creature, queue::ActionQueue, relic::Relic};

pub struct Player {
    pub creature: Creature,
    pub master_deck: CardPile,
    pub relics: Vec<Box<dyn Relic>>,
}

impl Player {
    pub fn trigger_relics_pre_combat(&mut self, queue: &mut ActionQueue) {
        for r in &mut self.relics {
            r.pre_combat(queue);
        }
    }
    pub fn trigger_relics_combat_start_pre_draw(&mut self, queue: &mut ActionQueue) {
        for r in &mut self.relics {
            r.combat_start_pre_draw(queue);
        }
    }
    pub fn trigger_relics_combat_start_post_draw(&mut self, queue: &mut ActionQueue) {
        for r in &mut self.relics {
            r.combat_start_post_draw(queue);
        }
    }
    pub fn trigger_relics_turn_end(&mut self, queue: &mut ActionQueue) {
        for r in &mut self.relics {
            r.turn_end(queue);
        }
    }
    pub fn trigger_relics_combat_finish(&mut self, queue: &mut ActionQueue) {
        for r in &mut self.relics {
            r.combat_finish(queue);
        }
    }
}
