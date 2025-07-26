use crate::{
    action::Action,
    creature::Creature,
    game::{CreatureRef, Game},
    status::Status,
};

pub fn calculate_modified_block(amount: i32, c: &Creature) -> i32 {
    if c.statuses.contains_key(&Status::NoBlock) {
        return 0;
    }
    let mut amount_f = amount as f32;
    if let Some(&s) = c.statuses.get(&Status::Dexterity) {
        amount_f += s as f32;
    }
    if c.statuses.contains_key(&Status::Frail) {
        amount_f *= 0.75;
    }
    amount_f as i32
}

pub struct BlockAction {
    target: CreatureRef,
    amount: i32,
    modify: bool,
}

impl BlockAction {
    pub fn player_card(amount: i32) -> Self {
        Self {
            target: CreatureRef::player(),
            amount,
            modify: true,
        }
    }
    pub fn player_flat_amount(amount: i32) -> Self {
        Self {
            target: CreatureRef::player(),
            amount,
            modify: false,
        }
    }
    pub fn monster(target: CreatureRef, amount: i32) -> Self {
        Self {
            target,
            amount,
            modify: false,
        }
    }
}

impl Action for BlockAction {
    fn run(&self, game: &mut Game) {
        let mut amount = self.amount;
        if self.modify {
            amount = calculate_modified_block(amount, &game.player.creature);
        }
        game.get_creature_mut(self.target).block += amount
    }
}

impl std::fmt::Debug for BlockAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "block {} {:?}", self.amount, self.target)
    }
}
