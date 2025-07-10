use crate::{actions::heal::HealAction, game::CreatureRef, queue::ActionQueue, relic::Relic};

pub struct BloodVial();

impl Relic for BloodVial {
    fn combat_start_post_draw(&mut self, queue: &mut ActionQueue) {
        queue.push_bot(HealAction {
            target: CreatureRef::player(),
            amount: 2,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{game::GameBuilder, relics::blood_vial::BloodVial};

    #[test]
    fn test_blood_vial() {
        let g = GameBuilder::default()
            .add_relic(BloodVial())
            .set_player_hp(50)
            .build_combat();
        assert_eq!(g.player.creature.cur_hp, 52);
    }
}
