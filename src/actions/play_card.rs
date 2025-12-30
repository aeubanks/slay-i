use rand::Rng;

use crate::{
    action::Action,
    actions::{
        clear_cur_card::ClearCurCardAction, damage::DamageAction, exhaust_card::ExhaustCardAction,
        gain_status::GainStatusAction,
    },
    card::{Card, CardPlayInfo, CardRef},
    cards::{CardClass, CardCost, CardType},
    game::{CreatureRef, Game},
    relic::RelicClass,
    status::Status,
};

use super::discard_card::DiscardCardAction;

pub struct PlayCardAction {
    pub card: CardRef,
    pub target: Option<CreatureRef>,
    pub is_duplicated: bool,
    pub cost: i32,
    pub free: bool,
    pub force_exhaust: bool,
    _priv: (),
}

impl PlayCardAction {
    pub fn duplicated(play: &PlayCardAction) -> Self {
        Self {
            is_duplicated: true,
            free: true,
            force_exhaust: false,
            card: play.card.clone(),
            target: play.target,
            cost: play.cost,
            _priv: (),
        }
    }
    fn is_corruption(game: &Game, card: &Card) -> bool {
        game.player.has_status(Status::Corruption) && card.class.ty() == CardType::Skill
    }
    pub fn new_free(
        card: CardRef,
        target: Option<CreatureRef>,
        game: &Game,
        force_exhaust: bool,
    ) -> Self {
        Self {
            free: true,
            force_exhaust,
            ..Self::new(card, target, game)
        }
    }
    pub fn new(card: CardRef, target: Option<CreatureRef>, game: &Game) -> Self {
        let cost = match card.borrow().cost {
            CardCost::Zero => 0,
            CardCost::X => game.energy,
            CardCost::Cost {
                base_cost,
                temporary_cost,
                free_to_play_once,
            } => {
                if free_to_play_once || PlayCardAction::is_corruption(game, &card.borrow()) {
                    0
                } else {
                    temporary_cost.unwrap_or(base_cost)
                }
            }
        };
        Self {
            card: card.clone(),
            target,
            is_duplicated: false,
            cost,
            free: false,
            force_exhaust: false,
            _priv: (),
        }
    }
}

impl Action for PlayCardAction {
    fn run(&self, game: &mut Game) {
        let c = self.card.borrow();
        assert!(game.can_play_card(self));

        game.num_cards_played_this_turn += 1;
        game.cur_card = Some(self.card.clone());

        for h in &game.hand {
            if h.borrow().class == CardClass::Pain {
                game.action_queue
                    .push_top(DamageAction::lose_hp(1, CreatureRef::player()));
            }
        }

        assert!(self.free || self.cost <= game.energy);
        let info = CardPlayInfo {
            card: &c,
            target: self.target,
            upgraded: c.upgrade_count != 0,
            upgrade_count: c.upgrade_count,
            base_increase: c.base_increase,
            cost: self.cost,
        };
        (c.class.behavior())(game, &info);

        enum CardDestination {
            Discard,
            Exhaust,
            None,
        }
        let mut dest = if self.is_duplicated || c.class.ty() == CardType::Power {
            CardDestination::None
        } else if c.exhaust || self.force_exhaust || PlayCardAction::is_corruption(game, &c) {
            CardDestination::Exhaust
        } else {
            CardDestination::Discard
        };
        drop(c);

        if matches!(dest, CardDestination::Exhaust)
            && game.has_relic(RelicClass::StrangeSpoon)
            && game.rng.random_range(0..=1) == 0
        {
            dest = CardDestination::Discard;
        }

        if let CardCost::Cost {
            free_to_play_once, ..
        } = &mut self.card.borrow_mut().cost
        {
            *free_to_play_once = false
        }

        game.player.trigger_statuses_on_card_played(
            &mut game.action_queue,
            &mut game.card_queue,
            self,
        );
        game.trigger_relics_on_card_played(self);
        for m in game.get_alive_monsters() {
            if let Some(amount) = game.get_creature(m).get_status(Status::Enrage) {
                game.action_queue.push_top(GainStatusAction {
                    status: Status::Strength,
                    amount,
                    target: m,
                });
            }
            if let Some(amount) = game.get_creature(m).get_status(Status::SharpHide) {
                game.action_queue.push_top(DamageAction::thorns_no_rupture(
                    amount,
                    CreatureRef::player(),
                ));
            }
        }
        if !self.free {
            game.energy -= self.cost;
        }
        game.action_queue.push_bot(ClearCurCardAction());
        match dest {
            CardDestination::None => {}
            CardDestination::Discard => game
                .action_queue
                .push_bot(DiscardCardAction(self.card.clone())),
            CardDestination::Exhaust => game
                .action_queue
                .push_bot(ExhaustCardAction(self.card.clone())),
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
    use crate::{cards::CardClass, combat::PlayCardStep, game::GameBuilder};

    #[test]
    fn test_play_attack_skill() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::TestAttack)
            .add_card(CardClass::TestSkill)
            .build_combat();

        assert_eq!(g.hand.len(), 2);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);

        g.step_test(PlayCardStep {
            hand_index: 0,
            target: None,
        });

        assert_eq!(g.hand.len(), 1);
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.draw_pile.len(), 0);

        g.step_test(PlayCardStep {
            hand_index: 0,
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

        g.step_test(PlayCardStep {
            hand_index: 0,
            target: None,
        });

        assert_eq!(g.hand.len(), 0);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);
    }
}
