use crate::{
    action::Action,
    game::{CreatureRef, Game},
    status::Status,
};

pub struct SetStatusAction {
    pub status: Status,
    pub target: CreatureRef,
}

impl Action for SetStatusAction {
    fn run(&self, game: &mut Game) {
        debug_assert!(
            !game
                .get_creature(self.target)
                .statuses
                .contains_key(&self.status)
                || game.get_creature(self.target).statuses.get(&self.status) == Some(&1)
        );
        game.get_creature_mut(self.target)
            .statuses
            .insert(self.status, 1);
    }
}

impl std::fmt::Debug for SetStatusAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gain {:?} {:?}", self.status, self.target)
    }
}
