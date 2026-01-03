use crate::{
    action::Action,
    actions::heal::HealAction,
    game::{CombatType, CreatureRef, Game},
    potion::Potion,
    relic::RelicClass,
};

pub struct UsePotionAction {
    pub potion: Potion,
    pub target: Option<CreatureRef>,
}

impl Action for UsePotionAction {
    fn run(&self, game: &mut Game) {
        if matches!(game.in_combat, CombatType::None) {
            assert!(self.potion.can_use_outside_combat());
        }
        let is_sacred = game.has_relic(RelicClass::SacredBark);
        self.potion.behavior()(is_sacred, self.target, game);
        if game.has_relic(RelicClass::ToyOrnithopter) {
            game.action_queue.push_bot(HealAction {
                target: CreatureRef::player(),
                amount: 5,
            });
        }
    }
}

impl std::fmt::Debug for UsePotionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "use potion {:?}", self.potion)?;
        if let Some(t) = self.target {
            write!(f, " on {t:?}")?
        }
        Ok(())
    }
}
