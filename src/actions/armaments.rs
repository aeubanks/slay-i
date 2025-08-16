use crate::{
    action::Action,
    game::{Game, GameState},
};

pub struct ArmamentsAction();

impl Action for ArmamentsAction {
    fn run(&self, game: &mut Game) {
        let upgradable = game
            .hand
            .iter()
            .filter(|c| c.borrow().can_upgrade())
            .collect::<Vec<_>>();
        match upgradable.len() {
            0 => {}
            1 => upgradable[0].borrow_mut().upgrade(),
            _ => game.state = GameState::Armaments,
        }
    }
}

impl std::fmt::Debug for ArmamentsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose upgrade one card in hand")
    }
}
