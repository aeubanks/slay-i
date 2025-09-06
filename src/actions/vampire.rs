use crate::{
    action::Action,
    actions::heal::HealAction,
    game::{CreatureRef, Game},
};

pub struct VampireAction(pub Vec<CreatureRef>);

impl Action for VampireAction {
    fn run(&self, game: &mut Game) {
        let heal = self
            .0
            .iter()
            .map(|c| game.get_creature(*c).last_damage_taken)
            .sum();
        if heal != 0 {
            game.action_queue.push_bot(HealAction {
                target: CreatureRef::player(),
                amount: heal,
            });
        }
    }
}

impl std::fmt::Debug for VampireAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vampire")
    }
}
