use std::ops::Deref;

use crate::{
    action::Action,
    actions::exhaust_card::ExhaustCardAction,
    card::{CardPlayInfo, CardRef},
    cards::{CardCost, CardType},
    game::{CreatureRef, Game},
};

use super::discard_card::DiscardCardAction;

pub struct PlayCardAction {
    pub card: CardRef,
    pub target: Option<CreatureRef>,
}

impl Action for PlayCardAction {
    fn run(&self, game: &mut Game) {
        let mut c = self.card.borrow_mut();
        let energy = match c.cost {
            CardCost::Zero => 0,
            CardCost::X => game.energy,
            CardCost::Cost {
                base_cost,
                temporary_cost,
            } => temporary_cost.unwrap_or(base_cost),
        };
        assert!(energy <= game.energy);
        let info = CardPlayInfo {
            upgraded: c.upgrade_count != 0,
            upgrade_count: c.upgrade_count,
            times_played: c.times_played,
        };
        (c.class.behavior())(game, self.target, info);
        c.times_played += 1;
        game.player
            .trigger_relics_on_card_played(&mut game.action_queue, c.deref());
        game.energy -= energy;
        if c.class.ty() == CardType::Power {
            return;
        }
        let exhaust = c.exhaust;
        drop(c);
        if exhaust {
            game.action_queue
                .push_bot(ExhaustCardAction(self.card.clone()));
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

#[cfg(test)]
mod tests {
    use crate::{
        cards::CardClass,
        game::{GameBuilder, Move},
    };

    #[test]
    fn test_play_attack_skill() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::TestAttack)
            .add_card(CardClass::TestSkill)
            .build_combat();

        assert_eq!(g.hand.len(), 2);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);

        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });

        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.draw_pile.len(), 0);

        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });

        assert_eq!(g.hand.len(), 0);
        assert_eq!(g.discard_pile.len(), 2);
        assert_eq!(g.draw_pile.len(), 0);
    }

    #[test]
    fn test_play_power() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::TestPower)
            .build_combat();

        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);

        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });

        assert_eq!(g.hand.len(), 0);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);
    }
}
