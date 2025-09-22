use crate::{
    action::Action,
    actions::gain_status::GainStatusAction,
    cards::CardType,
    game::{CreatureRef, Game},
    status::Status,
};

pub struct DuvuAction();

impl Action for DuvuAction {
    fn run(&self, game: &mut Game) {
        let amount = game
            .master_deck
            .iter()
            .filter(|c| c.borrow().class.ty() == CardType::Curse)
            .count() as i32;
        game.action_queue.push_top(GainStatusAction {
            status: Status::Strength,
            amount,
            target: CreatureRef::player(),
        });
    }
}

impl std::fmt::Debug for DuvuAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "duvu")
    }
}
