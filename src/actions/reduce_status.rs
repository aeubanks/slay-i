use crate::{
    action::Action,
    game::{CreatureRef, Game},
    status::{Status, StatusType},
};

pub struct ReduceStatusAction {
    pub status: Status,
    pub amount: i32,
    pub target: CreatureRef,
}

impl Action for ReduceStatusAction {
    fn run(&self, game: &mut Game) {
        assert!(self.amount > 0);
        // amount statuses like strength should go through negative GainStatusActions
        assert_ne!(self.status.ty(), StatusType::Amount);
        let c = game.get_creature_mut(self.target);
        if let Some(mut s) = c.get_status(self.status) {
            s -= self.amount;
            assert!(s >= 0);
            if s == 0 {
                c.remove_status(self.status);
            } else {
                c.set_status(self.status, s);
            }
        }
    }
}

impl std::fmt::Debug for ReduceStatusAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "reduce {} {:?} {:?}",
            self.amount, self.status, self.target
        )
    }
}
