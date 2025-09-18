use crate::{
    action::Action,
    actions::damage_random_monster::DamageRandomMonsterAction,
    creature::Creature,
    game::{CreatureRef, Game},
    status::Status,
};

pub fn calculate_modified_block(amount: i32, c: &Creature) -> i32 {
    if c.has_status(Status::NoBlock) {
        return 0;
    }
    let mut amount_f = amount as f32;
    if let Some(s) = c.get_status(Status::Dexterity) {
        amount_f += s as f32;
    }
    if c.has_status(Status::Frail) {
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
            amount = calculate_modified_block(amount, &game.player);
        }
        let c = game.get_creature_mut(self.target);
        c.block += amount;
        if c.block > 999 {
            c.block = 999;
        }
        if amount > 0
            && self.target.is_player()
            && let Some(j) = game.player.get_status(Status::Juggernaut)
        {
            game.action_queue.push_bot(DamageRandomMonsterAction {
                amount: j,
                thorns: true,
            });
        }
    }
}

impl std::fmt::Debug for BlockAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "block {} {:?}", self.amount, self.target)
    }
}

#[cfg(test)]
mod tests {
    use crate::{actions::block::BlockAction, game::GameBuilder};

    #[test]
    fn test_999() {
        let mut g = GameBuilder::default().build_combat();
        g.run_action(BlockAction::player_flat_amount(1000));
        assert_eq!(g.player.block, 999);
        g.run_action(BlockAction::player_card(1000));
        assert_eq!(g.player.block, 999);
    }
}
