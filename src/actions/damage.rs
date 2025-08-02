use crate::{
    action::Action,
    creature::Creature,
    game::{CreatureRef, Game},
    player::Player,
    status::Status,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DamageType {
    Attack { source: CreatureRef },
    Thorns { procs_rupture: bool },
    HPLoss,
}

pub struct DamageAction {
    target: CreatureRef,
    amount: i32,
    ty: DamageType,
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
            ty: DamageType::Attack {
                source: CreatureRef::player(),
            },
        }
    }
    pub fn from_monster(
        base_amount: i32,
        player: &Player,
        source: &Creature,
        source_ref: CreatureRef,
    ) -> Self {
        let target = CreatureRef::player();
        let amount = calculate_damage(base_amount, false, source, player);
        Self {
            target,
            amount,
            ty: DamageType::Attack { source: source_ref },
        }
    }
    pub fn thorns_rupture(amount: i32, target: CreatureRef) -> Self {
        Self {
            target,
            amount,
            ty: DamageType::Thorns {
                procs_rupture: true,
            },
        }
    }
    pub fn thorns_no_rupture(amount: i32, target: CreatureRef) -> Self {
        Self {
            target,
            amount,
            ty: DamageType::Thorns {
                procs_rupture: false,
            },
        }
    }
    pub fn lose_hp(amount: i32, target: CreatureRef) -> Self {
        Self {
            target,
            amount,
            ty: DamageType::HPLoss,
        }
    }
}

impl Action for DamageAction {
    fn run(&self, game: &mut Game) {
        game.damage(self.target, self.amount, self.ty);
    }
}

impl std::fmt::Debug for DamageAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "damage {} hp {:?} ({:?})",
            self.amount, self.target, self.ty
        )
    }
}
