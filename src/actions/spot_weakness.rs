use crate::{
    action::Action,
    actions::gain_status::GainStatusAction,
    game::{CreatureRef, Game},
    status::Status,
};

pub struct SpotWeaknessAction {
    pub target: CreatureRef,
    pub amount: i32,
}

impl Action for SpotWeaknessAction {
    fn run(&self, game: &mut Game) {
        if game.monsters[self.target.monster_index()]
            .behavior
            .get_intent()
            .is_attack()
        {
            game.action_queue.push_top(GainStatusAction {
                status: Status::Strength,
                amount: self.amount,
                target: CreatureRef::player(),
            });
        }
    }
}

impl std::fmt::Debug for SpotWeaknessAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "noop")
    }
}
