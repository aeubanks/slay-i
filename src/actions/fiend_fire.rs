use crate::{
    action::Action,
    actions::{damage::DamageAction, exhaust_hand::ExhaustHandAction},
    game::{CreatureRef, Game},
};

pub struct FiendFireAction {
    pub target: CreatureRef,
    pub amount: i32,
}

impl Action for FiendFireAction {
    fn run(&self, game: &mut Game) {
        let count = game.hand.len();
        for _ in 0..count {
            game.action_queue.push_top(DamageAction::from_player(
                self.amount,
                &game.player,
                game.get_creature(self.target),
                self.target,
            ));
        }
        game.action_queue.push_top(ExhaustHandAction());
    }
}

impl std::fmt::Debug for FiendFireAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fiend fire")
    }
}
