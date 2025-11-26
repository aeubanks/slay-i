use crate::{
    action::Action,
    actions::{play_card::PlayCardAction, shuffle_discard_into_draw::ShuffleDiscardIntoDrawAction},
    game::Game,
};

pub struct PlayTopCardAction {
    pub force_exhaust: bool,
}

impl Action for PlayTopCardAction {
    fn run(&self, g: &mut Game) {
        if g.draw_pile.is_empty() && g.discard_pile.is_empty() {
            return;
        }
        if g.draw_pile.is_empty() {
            g.action_queue.push_top(PlayTopCardAction { ..*self });
            g.action_queue.push_top(ShuffleDiscardIntoDrawAction());
            return;
        }
        let c = g.draw_pile.pop(&mut g.rng);
        let target = if c.borrow().has_target() {
            Some(g.get_random_alive_monster())
        } else {
            None
        };
        g.card_queue
            .push(PlayCardAction::new_free(c, target, g, self.force_exhaust));
    }
}

impl std::fmt::Debug for PlayTopCardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "play top card")?;
        if self.force_exhaust {
            write!(f, " (force exhaust)")?;
        }
        Ok(())
    }
}
