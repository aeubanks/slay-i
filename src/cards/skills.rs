use crate::{
    actions::block::BlockAction,
    card::CardPlayInfo,
    game::{CreatureRef, Game},
};

pub fn defend_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    game.action_queue.push_bot(BlockAction {
        target: CreatureRef::player(),
        amount: if info.upgraded { 8 } else { 5 },
    });
}

pub fn ghostly_armor_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    game.action_queue.push_bot(BlockAction {
        target: CreatureRef::player(),
        amount: if info.upgraded { 13 } else { 10 },
    });
}

pub fn impervious_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    game.action_queue.push_bot(BlockAction {
        target: CreatureRef::player(),
        amount: if info.upgraded { 40 } else { 30 },
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        cards::{CardClass, card, upgraded_card},
        game::{GameBuilder, Move},
    };

    #[test]
    fn test_defend() {
        let mut g = GameBuilder::default()
            .add_card(card(CardClass::Defend))
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);
        assert_eq!(g.player.creature.block, 5);
    }

    #[test]
    fn test_upgraded_defend() {
        let mut g = GameBuilder::default()
            .add_card(upgraded_card(CardClass::Defend))
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.discard_pile.len(), 1);
        assert_eq!(g.exhaust_pile.len(), 0);
        assert_eq!(g.draw_pile.len(), 0);
        assert_eq!(g.player.creature.block, 8);
    }

    #[test]
    fn test_impervious() {
        let mut g = GameBuilder::default()
            .add_card(card(CardClass::Impervious))
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.discard_pile.len(), 0);
        assert_eq!(g.exhaust_pile.len(), 1);
        assert_eq!(g.draw_pile.len(), 0);
        assert_eq!(g.player.creature.block, 30);
    }
}
