use crate::{
    action::Action,
    actions::damage::DamageAction,
    game::{CreatureRef, Game},
};

pub struct DamageAllMonstersAction {
    amount: i32,
    thorns: bool,
}

impl DamageAllMonstersAction {
    pub fn from_player(amount: i32) -> Self {
        Self {
            amount,
            thorns: false,
        }
    }
    pub fn thorns(amount: i32) -> Self {
        Self {
            amount,
            thorns: false,
        }
    }
}

impl Action for DamageAllMonstersAction {
    fn run(&self, game: &mut Game) {
        for (mi, m) in game.monsters.iter().enumerate() {
            if !m.creature.is_alive() {
                continue;
            }
            let target = CreatureRef::monster(mi);
            game.action_queue.push_top(if self.thorns {
                DamageAction::thorns_no_rupture(self.amount, target)
            } else {
                DamageAction::from_player(
                    self.amount,
                    &game.player,
                    &game.monsters[mi].creature,
                    target,
                )
            });
        }
    }
}

impl std::fmt::Debug for DamageAllMonstersAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "damage all monsters {} hp ({})",
            self.amount,
            if self.thorns { "thorns" } else { "player" }
        )
    }
}
