use crate::{actions::draw::DrawAction, queue::ActionQueue, relic::Relic};

pub struct BagOfPrep();

impl Relic for BagOfPrep {
    fn combat_start_post_draw(&mut self, queue: &mut ActionQueue) {
        queue.push_bot(DrawAction(2));
    }
}

#[cfg(test)]
mod tests {
    use crate::{game::GameBuilder, relics::bag_of_prep::BagOfPrep};

    #[test]
    fn test_bag_of_prep() {
        let g = GameBuilder::default()
            .ironclad_starting_deck()
            .add_relic(BagOfPrep())
            .build_combat();
        assert_eq!(g.hand.len(), 7);
    }
}
