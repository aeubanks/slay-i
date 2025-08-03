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
        if self.status == Status::NoDraw && c.statuses.contains_key(&Status::NoDraw) {
            return;
        }
        if self.status.is_debuff(self.amount) && c.statuses.contains_key(&Status::Artifact) {
            game.action_queue.push_top(ReduceStatusAction {
                status: Status::Artifact,
                amount: 1,
                target: self.target,
            });
            return;
        }
        if self.status.does_not_stack() {
            let c = game.get_creature(self.target);
            if c.statuses.contains_key(&self.status) {
                return;
            }
        }
        let extra = self.target.is_player()
            && game.should_add_extra_decay_status()
            && self.status.decays()
            && !game.player.creature.statuses.contains_key(&self.status);
        let v = game
            .get_creature_mut(self.target)
            .statuses
            .entry(self.status)
            .or_default();
        *v += self.amount;
        if extra {
            *v += 1;
        }
        if *v == 0 {
            game.get_creature_mut(self.target)
                .statuses
                .remove(&self.status);
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
