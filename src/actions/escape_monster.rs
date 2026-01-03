use crate::{
    action::Action,
    creature::CreatureState,
    game::{CreatureRef, Game},
};

pub struct EscapeMonsterAction(pub CreatureRef);

impl Action for EscapeMonsterAction {
    fn run(&self, game: &mut Game) {
        game.get_creature_mut(self.0).state = CreatureState::Escaped;
    }
}

impl std::fmt::Debug for EscapeMonsterAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "escape {:?}", self.0)
    }
}
