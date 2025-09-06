use crate::{
    action::Action,
    actions::reduce_status::ReduceStatusAction,
    game::{CreatureRef, Game},
    status::{Status, StatusType},
};

pub struct GainStatusAction {
    pub status: Status,
    pub amount: i32,
    pub target: CreatureRef,
}

impl Action for GainStatusAction {
    fn run(&self, game: &mut Game) {
        if self.status.does_not_stack() {
            assert_eq!(self.amount, 1);
        }
        if self.status.ty() != StatusType::Amount {
            assert!(self.amount > 0);
        }
        let c = game.get_creature(self.target);
        if self.status == Status::NoDraw && c.has_status(Status::NoDraw) {
            return;
        }
        if self.status.is_debuff(self.amount) && c.has_status(Status::Artifact) {
            game.action_queue.push_top(ReduceStatusAction {
                status: Status::Artifact,
                amount: 1,
                target: self.target,
            });
            return;
        }
        if self.status.does_not_stack() {
            let c = game.get_creature(self.target);
            if c.has_status(self.status) {
                return;
            }
        }
        let extra = self.target.is_player()
            && game.should_add_extra_decay_status()
            && self.status.decays()
            && !game.player.creature.has_status(self.status);
        let creature = game.get_creature_mut(self.target);
        let mut v = creature.get_status(self.status).unwrap_or(0);
        v += self.amount;
        if extra {
            v += 1;
        }
        if self.status.bounded_999() {
            v = v.clamp(-999, 999);
        }
        if v == 0 {
            creature.remove_status(self.status);
        } else {
            creature.set_status(self.status, v);
        }
    }
}

impl std::fmt::Debug for GainStatusAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "gain {} {:?} {:?}",
            self.amount, self.status, self.target
        )
    }
}
