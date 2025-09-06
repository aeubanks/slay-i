use crate::{
    action::Action,
    game::{CreatureRef, Game},
    status::Status,
};

pub struct RemoveStatusAction {
    pub status: Status,
    pub target: CreatureRef,
}

impl Action for RemoveStatusAction {
    fn run(&self, game: &mut Game) {
        game.get_creature_mut(self.target)
            .remove_status(self.status);
    }
}

impl std::fmt::Debug for RemoveStatusAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "remove {:?} {:?}", self.status, self.target)
    }
}
