use crate::{
    action::Action,
    game::{CreatureRef, Game},
};

pub struct BlockAction {
    pub target: CreatureRef,
    pub amount: i32,
}

impl Action for BlockAction {
    fn run(&self, game: &mut Game) {
        game.get_creature_mut(self.target).block += self.amount
    }
}

impl std::fmt::Debug for BlockAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "block {} {:?}", self.amount, self.target)
    }
}
