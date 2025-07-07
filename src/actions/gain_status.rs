use crate::{
    action::Action,
    game::{CreatureRef, Game},
    status::Status,
};

pub struct GainStatusAction {
    pub status: Status,
    pub amount: i32,
    pub target: CreatureRef,
}

impl Action for GainStatusAction {
    fn run(&self, game: &mut Game) {
        let extra = self.target.is_player()
            && game.is_monster_turn()
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
