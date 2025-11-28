use crate::{
    action::Action, actions::exhaust_card::ExhaustCardAction, game::Game, state::GameState,
    step::Step,
};

pub struct ChooseCardInHandToExhaustAction();

impl Action for ChooseCardInHandToExhaustAction {
    fn run(&self, game: &mut Game) {
        match game.hand.len() {
            0 => {}
            1 => game
                .action_queue
                .push_top(ExhaustCardAction(game.hand.pop().unwrap())),
            _ => game.state.push_state(ChooseExhaustOneCardInHandGameState),
        }
    }
}

impl std::fmt::Debug for ChooseCardInHandToExhaustAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose card in hand to exhaust")
    }
}

#[derive(Debug)]
struct ChooseExhaustOneCardInHandGameState;

impl GameState for ChooseExhaustOneCardInHandGameState {
    fn valid_steps(&self, game: &Game) -> Option<Vec<Box<dyn Step>>> {
        let mut moves = Vec::<Box<dyn Step>>::new();
        for i in 0..game.hand.len() {
            moves.push(Box::new(ExhaustOneCardInHandStep { hand_index: i }));
        }
        Some(moves)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ExhaustOneCardInHandStep {
    pub hand_index: usize,
}

impl Step for ExhaustOneCardInHandStep {
    fn run(&self, game: &mut Game) {
        game.action_queue
            .push_top(ExhaustCardAction(game.hand.remove(self.hand_index)));
    }

    fn description(&self, game: &Game) -> String {
        format!(
            "exhaust {} ({:?})",
            self.hand_index,
            game.hand[self.hand_index].borrow()
        )
    }
}
