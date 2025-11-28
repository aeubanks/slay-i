use crate::{
    action::Action, actions::upgrade::UpgradeAction, game::Game, state::GameState, step::Step,
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
            _ => game.state.push_state(ChooseArmamentsGameState),
        }
    }
}

impl std::fmt::Debug for ArmamentsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose upgrade one card in hand")
    }
}

#[derive(Debug)]
struct ChooseArmamentsGameState;

impl GameState for ChooseArmamentsGameState {
    fn valid_steps(&self, game: &Game) -> Option<Vec<Box<dyn Step>>> {
        let mut moves = Vec::<Box<dyn Step>>::new();
        for (i, c) in game.hand.iter().enumerate() {
            if c.borrow().can_upgrade() {
                moves.push(Box::new(ArmamentsStep { hand_index: i }));
            }
        }
        Some(moves)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ArmamentsStep {
    pub hand_index: usize,
}

impl Step for ArmamentsStep {
    fn run(&self, game: &mut Game) {
        game.action_queue
            .push_top(UpgradeAction(game.hand[self.hand_index].clone()));
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "upgrade card {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        )
    }
}
