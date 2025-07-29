use crate::{
    action::Action,
    actions::{block::BlockAction, draw::DrawAction},
    card::CardRef,
    game::Game,
    status::Status,
};

pub struct ExhaustCardAction {
    pub card: CardRef,
}

impl Action for ExhaustCardAction {
    fn run(&self, game: &mut Game) {
        self.card.borrow_mut().clear_temporary();
        game.exhaust_pile.push(self.card.clone());
        if let Some(a) = game.player.creature.statuses.get(&Status::FeelNoPain) {
            game.action_queue
                .push_bot(BlockAction::player_flat_amount(*a));
        }
        if let Some(a) = game.player.creature.statuses.get(&Status::DarkEmbrace) {
            game.action_queue.push_bot(DrawAction(*a));
        }
    }
}

impl std::fmt::Debug for ExhaustCardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exhaust {:?}", self.card)
    }
}
