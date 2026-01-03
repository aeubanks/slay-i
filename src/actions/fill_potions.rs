use crate::{
    action::Action,
    game::Game,
    potion::{Potion, random_potion_weighted},
};

pub struct FillPotionsAction();

impl Action for FillPotionsAction {
    fn run(&self, game: &mut Game) {
        for p in &mut game.potions {
            if p.is_none() {
                let mut roll;
                loop {
                    roll = random_potion_weighted(&mut game.rng);
                    if !game.in_combat || roll != Potion::Fruit {
                        break;
                    }
                }
                *p = Some(roll);
            }
        }
    }
}

impl std::fmt::Debug for FillPotionsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fill potions")
    }
}
