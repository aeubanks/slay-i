use crate::{actions::heal::HealAction, game::CreatureRef, queue::ActionQueue, relic::Relic};

pub struct BurningBlood {}

impl Relic for BurningBlood {
    fn combat_finish(&mut self, queue: &mut ActionQueue) {
        queue.push_bot(HealAction {
            target: CreatureRef::player(),
            amount: 6,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cards::{CardClass, new_card},
        game::{GameBuilder, Move},
        relics::burning_blood::BurningBlood,
    };

    #[test]
    fn test_burning_blood() {
        let mut g = GameBuilder::default()
            .add_card(new_card(CardClass::DebugKill))
            .add_relic(BurningBlood {})
            .build_combat();
        let hp = g.player.creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.player.creature.cur_hp, hp + 6);
    }
}
