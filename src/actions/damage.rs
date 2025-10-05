use crate::{
    action::Action,
    actions::{
        gain_gold::GainGoldAction, increase_base_amount::IncreaseBaseAmountAction,
        increase_max_hp::IncreaseMaxHPAction,
    },
    game::{CreatureRef, Game},
    queue::ActionQueue,
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

impl DamageAction {
    pub fn from_player(base_amount: i32, target: CreatureRef) -> Self {
        Self {
            target,
            amount: base_amount,
            ty: DamageType::Attack {
                source: CreatureRef::player(),
                on_fatal: None,
            },
        }
    }
    pub fn from_player_with_on_fatal(
        base_amount: i32,
        target: CreatureRef,
        on_fatal: OnFatal,
    ) -> Self {
        Self {
            target,
            amount: base_amount,
            ty: DamageType::Attack {
                source: CreatureRef::player(),
                on_fatal: Some(on_fatal),
            },
        }
    }
    pub fn from_monster(base_amount: i32, source: CreatureRef) -> Self {
        Self {
            target: CreatureRef::player(),
            amount: base_amount,
            ty: DamageType::Attack {
                source,
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
