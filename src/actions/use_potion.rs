use crate::{
    action::Action,
    game::{CreatureRef, Game},
    potion::Potion,
    relic::RelicClass,
};

pub struct UsePotionAction {
    pub potion: Potion,
    pub target: Option<CreatureRef>,
}

impl Action for UsePotionAction {
    fn run(&self, game: &mut Game) {
        let is_sacred = game.has_relic(RelicClass::SacredBark);
        self.potion.behavior()(is_sacred, self.target, game);
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
