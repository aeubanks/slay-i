use crate::{
    action::Action,
    actions::exhaust_card::ExhaustCardAction,
    card::{CardPlayInfo, CardRef},
    game::{CreatureRef, Game},
};

use super::discard_card::DiscardCardAction;

pub struct PlayCardAction {
    pub card: CardRef,
    pub target: Option<CreatureRef>,
}

impl Action for PlayCardAction {
    fn run(&self, game: &mut Game) {
        let c = self.card.borrow_mut();
        let energy = c.cost;
        assert!(energy <= game.energy);
        game.energy -= energy;
        let info = CardPlayInfo {
            upgraded: c.upgrade_count != 0,
            upgrade_count: c.upgrade_count,
            played_count: 0,
        };
        (c.behavior)(game, self.target, info);
        let exhaust = c.exhaust;
        drop(c);
        if exhaust {
            game.action_queue.push_bot(ExhaustCardAction {
                card: self.card.clone(),
            });
        } else {
            game.action_queue.push_bot(DiscardCardAction {
                card: self.card.clone(),
            });
        }
    }
}

impl std::fmt::Debug for PlayCardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "play {:?}", self.card.borrow())?;
        if let Some(t) = self.target {
            write!(f, " on {t:?}")?
        }
        Ok(())
    }
}
