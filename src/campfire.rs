use crate::{
    actions::heal::HealAction,
    game::{ChooseUpgradeMasterGameState, CreatureRef, Game, RunActionsGameState},
    step::Step,
};

#[derive(Debug, PartialEq, Eq)]
pub struct CampfireRestStep;

impl Step for CampfireRestStep {
    fn run(&self, game: &mut Game) {
        game.action_queue.push_bot(HealAction {
            target: CreatureRef::player(),
            amount: (game.player.max_hp as f32 * 0.3) as i32,
        });
        game.state.push_state(RunActionsGameState);
    }

    fn description(&self, _: &Game) -> String {
        "campfire rest".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CampfireUpgradeStep;

impl Step for CampfireUpgradeStep {
    fn run(&self, game: &mut Game) {
        game.state.push_state(ChooseUpgradeMasterGameState);
    }

    fn description(&self, _: &Game) -> String {
        "campfire upgrade".to_owned()
    }
}
