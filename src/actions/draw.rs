use rand::Rng;

use crate::{
    action::Action,
    actions::{
        damage_all_monsters::DamageAllMonstersAction, gain_energy::GainEnergyAction,
        shuffle_discard_on_top_of_draw::ShuffleDiscardOnTopOfDrawAction,
    },
    cards::{CardClass, CardCost, CardType},
    game::Game,
    status::Status,
};

pub struct DrawAction(pub i32);

impl Action for DrawAction {
    fn run(&self, game: &mut Game) {
        if game.player.has_status(Status::NoDraw) || game.all_monsters_dead() {
            return;
        }
        let discard_size = game.discard_pile.len() as i32;
        let draw_size = game.draw_pile.len() as i32;
        let hand_size = game.hand.len() as i32;

        if draw_size == 0 && discard_size == 0 {
            return;
        }

        let mut amount = self.0.min(Game::MAX_HAND_SIZE - hand_size);
        if amount == 0 {
            return;
        }

        if amount > draw_size {
            game.action_queue.push_top(DrawAction(amount - draw_size));
            game.action_queue
                .push_top(ShuffleDiscardOnTopOfDrawAction());
            amount = draw_size;
        }

        for _ in 0..amount {
            let c = game.draw_pile.pop().unwrap();
            {
                let mut c = c.borrow_mut();
                if game.player.has_status(Status::Confusion)
                    && let CardCost::Cost {
                        base_cost,
                        temporary_cost,
                        free_to_play_once,
                    } = &mut c.cost
                {
                    *base_cost = game.rng.random_range(0..=3);
                    *temporary_cost = None;
                    *free_to_play_once = false;
                }
                let class = c.class;
                if class == CardClass::Void {
                    game.action_queue.push_bot(GainEnergyAction(-1));
                }
                if class.ty() == CardType::Status {
                    if let Some(v) = game.player.get_status(Status::FireBreathing) {
                        game.action_queue
                            .push_bot(DamageAllMonstersAction::thorns(v));
                    }
                    if let Some(v) = game.player.get_status(Status::Evolve) {
                        game.action_queue.push_bot(DrawAction(v));
                    }
                }
            }
            game.hand.push(c);
        }
    }
}

impl std::fmt::Debug for DrawAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "draw {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::draw::DrawAction,
        cards::CardClass,
        game::{CreatureRef, GameBuilder},
        relic::RelicClass,
    };

    #[test]
    fn test_shuffle() {
        let mut gb = GameBuilder::default();
        for _ in 0..12 {
            gb = gb.add_card(CardClass::Strike);
        }
        let mut g = gb.build_combat();

        assert_eq!(g.hand.len(), 5);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 7);

        g.run_action(DrawAction(1));

        assert_eq!(g.hand.len(), 6);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 6);

        g.run_action(DrawAction(5));

        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 2);

        g.discard_pile.push(g.hand.pop().unwrap());
        g.discard_pile.push(g.hand.pop().unwrap());

        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.discard_pile.len(), 2);
        assert_eq!(g.draw_pile.len(), 2);

        g.run_action(DrawAction(5));

        assert_eq!(g.hand.len(), 10);
        assert_eq!(g.discard_pile.len(), 2);
        assert_eq!(g.draw_pile.len(), 0);

        g.discard_pile.push(g.hand.pop().unwrap());
        g.discard_pile.push(g.hand.pop().unwrap());

        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.discard_pile.len(), 4);
        assert_eq!(g.draw_pile.len(), 0);

        g.run_action(DrawAction(1));

        assert_eq!(g.hand.len(), 9);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 3);

        g.draw_pile.clear();

        g.run_action(DrawAction(1));

        assert_eq!(g.hand.len(), 9);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);

        g.discard_pile.push(g.hand.pop().unwrap());
        g.discard_pile.push(g.hand.pop().unwrap());
        g.draw_pile.push(g.hand.pop().unwrap());
        g.draw_pile.push(g.hand.pop().unwrap());

        assert_eq!(g.hand.len(), 5);
        assert_eq!(g.discard_pile.len(), 2);
        assert_eq!(g.draw_pile.len(), 2);

        g.run_action(DrawAction(3));

        assert_eq!(g.hand.len(), 8);
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 1);
    }

    #[test]
    fn test_draw_monsters_dead() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::GremlinHorn)
            .add_relic(RelicClass::Sundial)
            .build_combat();
        g.add_card_to_discard_pile(CardClass::Strike);
        assert_eq!(g.get_relic_value(RelicClass::Sundial), Some(0));
        g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(0)));
        assert_eq!(g.get_relic_value(RelicClass::Sundial), Some(0));
    }
}
