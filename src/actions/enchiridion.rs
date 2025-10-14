use crate::{
    action::Action, actions::discovery::DiscoveryAction, cards::random_red_power_in_combat,
    game::Game,
};

pub struct EnchiridionAction();

impl Action for EnchiridionAction {
    fn run(&self, game: &mut Game) {
        let class = random_red_power_in_combat(&mut game.rng);
        game.action_queue
            .push_top(DiscoveryAction { class, amount: 1 });
    }
}

impl std::fmt::Debug for EnchiridionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "enchiridion")
    }
}
