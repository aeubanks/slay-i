use crate::{
    action::Action,
    actions::{
        gain_gold::GainGoldAction, increase_base_amount::IncreaseBaseAmountAction,
        increase_max_hp::IncreaseMaxHPAction,
    },
    creature::Creature,
    game::{CreatureRef, Game},
    player::Player,
    queue::ActionQueue,
    status::Status,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OnFatalType {
    Feed,
    HandOfGreed,
    RitualDagger { card_id: u32 },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OnFatal {
    pub ty: OnFatalType,
    pub upgraded: bool,
}

impl OnFatal {
    fn trigger(&self, queue: &mut ActionQueue) {
        match self.ty {
            OnFatalType::Feed => {
                queue.push_top(IncreaseMaxHPAction(if self.upgraded { 4 } else { 3 }))
            }
            OnFatalType::HandOfGreed => {
                queue.push_top(GainGoldAction(if self.upgraded { 25 } else { 20 }))
            }
            OnFatalType::RitualDagger { card_id } => queue.push_top(IncreaseBaseAmountAction {
                card_id,
                amount: if self.upgraded { 5 } else { 3 },
                master: true,
            }),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DamageType {
    Attack {
        source: CreatureRef,
        on_fatal: Option<OnFatal>,
    },
    Thorns {
        procs_rupture: bool,
    },
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
    if let Some(s) = source.get_status(Status::Strength) {
        amount_f += s as f32;
    }
    if source.has_status(Status::Weak) {
        amount_f *= 0.75;
    }
    if source.has_status(Status::PenNib) {
        amount_f *= 2.0;
    }
    if target.has_status(Status::Vulnerable) {
        amount_f *= 1.5;
    }
    0.max(amount_f as i32)
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
                on_fatal: None,
            },
        }
    }
    pub fn from_player_with_on_fatal(
        base_amount: i32,
        player: &Player,
        target: &Creature,
        target_ref: CreatureRef,
        on_fatal: OnFatal,
    ) -> Self {
        let amount = calculate_damage(base_amount, true, target, player);
        Self {
            target: target_ref,
            amount,
            ty: DamageType::Attack {
                source: CreatureRef::player(),
                on_fatal: Some(on_fatal),
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
            ty: DamageType::Attack {
                source: source_ref,
                on_fatal: None,
            },
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
        if !game.get_creature(self.target).is_alive() {
            return;
        }
        game.damage(self.target, self.amount, self.ty);
        if !game.get_creature(self.target).is_alive()
            && let DamageType::Attack {
                source: _,
                on_fatal,
            } = self.ty
            && let Some(on_fatal) = on_fatal
        {
            on_fatal.trigger(&mut game.action_queue);
        }
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
