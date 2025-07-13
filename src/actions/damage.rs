use crate::{
    action::Action,
    creature::Creature,
    game::{CreatureRef, Game},
    player::Player,
    status::Status,
};

pub struct DamageAction {
    // source: CreatureRef,
    target: CreatureRef,
    amount: i32,
}

pub fn calculate_damage(
    amount: i32,
    player_is_source: bool,
    monster: &Creature,
    player: &Player,
) -> i32 {
    let mut amount_f = amount as f32;
    let (source, target) = if player_is_source {
        (&player.creature, monster)
    } else {
        (monster, &player.creature)
    };
    if let Some(&s) = source.statuses.get(&Status::Strength) {
        amount_f += s as f32;
    }
    if source.statuses.contains_key(&Status::Weak) {
        amount_f *= 0.75;
    }
    if target.statuses.contains_key(&Status::Vulnerable) {
        amount_f *= 1.5;
    }
    amount_f as i32
}

impl DamageAction {
    pub fn from_player(
        base_amount: i32,
        player: &Player,
        target: &Creature,
        target_ref: CreatureRef,
    ) -> Self {
        let amount = calculate_damage(base_amount, true, target, player);
        Self {
            target: target_ref,
            amount,
        }
    }
    pub fn from_monster(
        base_amount: i32,
        player: &Player,
        source: &Creature,
        _source_ref: CreatureRef,
    ) -> Self {
        let target = CreatureRef::player();
        let amount = calculate_damage(base_amount, false, source, player);
        Self { target, amount }
    }
}

impl Action for DamageAction {
    fn run(&self, game: &mut Game) {
        let c = game.get_creature_mut(self.target);
        if !c.is_alive() {
            return;
        }
        if c.block >= self.amount {
            c.block -= self.amount;
        } else {
            c.cur_hp += c.block;
            c.cur_hp -= self.amount;
            if c.cur_hp < 0 {
                c.cur_hp = 0;
            }
            c.block = 0;
        }
    }
}

impl std::fmt::Debug for DamageAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "damage {} hp {:?}", self.amount, self.target)
    }
}
