use crate::{
    action::Action,
    creature::Creature,
    game::{CreatureRef, Game},
    status::Status,
};

pub fn calculate_player_block(amount: i32, player: &Creature) -> i32 {
    if player.statuses.contains_key(&Status::NoBlock) {
        return 0;
    }
    let mut amount_f = amount as f32;
    if let Some(&s) = player.statuses.get(&Status::Dexterity) {
        amount_f += s as f32;
    }
    if player.statuses.contains_key(&Status::Frail) {
        amount_f *= 0.75;
    }
    amount_f as i32
}

pub struct BlockAction {
    pub target: CreatureRef,
    pub amount: i32,
}

impl Action for BlockAction {
    fn run(&self, game: &mut Game) {
        let mut amount = self.amount;
        if self.target.is_player() {
            amount = calculate_player_block(amount, &game.player.creature);
        }
        game.get_creature_mut(self.target).block += amount
    }
}

impl std::fmt::Debug for BlockAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "block {} {:?}", self.amount, self.target)
    }
}
