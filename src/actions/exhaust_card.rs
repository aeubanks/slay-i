use crate::{
    action::Action,
    actions::{block::BlockAction, draw::DrawAction, gain_energy::GainEnergyAction},
    card::CardRef,
    cards::CardClass,
    game::Game,
    status::Status,
};

pub struct ExhaustCardAction {
    pub card: CardRef,
}

impl Action for ExhaustCardAction {
    fn run(&self, game: &mut Game) {
        if let Some(a) = game.player.creature.statuses.get(&Status::FeelNoPain) {
            game.action_queue
                .push_bot(BlockAction::player_flat_amount(*a));
        }
        if let Some(a) = game.player.creature.statuses.get(&Status::DarkEmbrace) {
            game.action_queue.push_bot(DrawAction(*a));
        }

        {
            let mut c = self.card.borrow_mut();
            c.clear_temporary();
            if c.class == CardClass::Sentinel {
                game.action_queue
                    .push_bot(GainEnergyAction(if c.upgrade_count == 0 { 2 } else { 3 }));
            }
        }

        game.exhaust_pile.push(self.card.clone());
    }
}

impl std::fmt::Debug for ExhaustCardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exhaust {:?}", self.card)
    }
}
