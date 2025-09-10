use crate::{
    action::Action,
    cards::{
        random_colorless_in_combat, random_red_attack_in_combat, random_red_in_combat,
        random_red_power_in_combat, random_red_skill_in_combat,
    },
    game::{Game, GameState},
};

pub enum ChooseDiscoveryType {
    Red,
    RedAttack,
    RedSkill,
    RedPower,
    Colorless,
}

pub struct ChooseDiscoveryAction {
    pub ty: ChooseDiscoveryType,
    pub amount: i32,
}

impl Action for ChooseDiscoveryAction {
    fn run(&self, game: &mut Game) {
        let mut classes = Vec::new();
        while classes.len() < 3 {
            let c = match self.ty {
                ChooseDiscoveryType::Red => random_red_in_combat(&mut game.rng),
                ChooseDiscoveryType::RedAttack => random_red_attack_in_combat(&mut game.rng),
                ChooseDiscoveryType::RedSkill => random_red_skill_in_combat(&mut game.rng),
                ChooseDiscoveryType::RedPower => random_red_power_in_combat(&mut game.rng),
                ChooseDiscoveryType::Colorless => random_colorless_in_combat(&mut game.rng),
            };
            if !classes.contains(&c) {
                classes.push(c);
            }
        }
        game.state = GameState::Discovery {
            classes,
            amount: self.amount,
        };
    }
}

impl std::fmt::Debug for ChooseDiscoveryAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "choose discovery")
    }
}
