use crate::{
    actions::gain_status::GainStatusAction,
    card::CardPlayInfo,
    game::{CreatureRef, Game},
    status::Status,
};

pub fn inflame_behavior(game: &mut Game, info: CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Strength,
        target: CreatureRef::player(),
        amount: if info.upgraded { 3 } else { 2 },
    });
}

pub fn feel_no_pain_behavior(game: &mut Game, info: CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::FeelNoPain,
        target: CreatureRef::player(),
        amount: if info.upgraded { 4 } else { 3 },
    });
}

pub fn dark_embrace_behavior(game: &mut Game, _: CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::DarkEmbrace,
        target: CreatureRef::player(),
        amount: 1,
    });
}

pub fn brutality_behavior(game: &mut Game, _: CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Brutality,
        target: CreatureRef::player(),
        amount: 1,
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::exhaust_card::ExhaustCardAction,
        cards::CardClass,
        game::{GameBuilder, Move},
        status::Status,
    };

    #[test]
    fn test_inflame() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Inflame)
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.player.creature.statuses.get(&Status::Strength), Some(&2));
    }

    #[test]
    fn test_feel_no_pain() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::FeelNoPain, 10)
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        let card = g.hand.pop().unwrap();
        g.run_action(ExhaustCardAction(card));
        assert_eq!(g.player.creature.block, 3);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        let card = g.hand.pop().unwrap();
        g.run_action(ExhaustCardAction(card));
        assert_eq!(g.player.creature.block, 9);
    }

    #[test]
    fn test_dark_embrace() {
        let mut g = GameBuilder::default()
            .add_cards_upgraded(CardClass::DarkEmbrace, 10)
            .build_combat();
        assert_eq!(g.hand.len(), 5);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.hand.len(), 4);
        let card = g.hand.pop().unwrap();
        assert_eq!(g.hand.len(), 3);
        g.run_action(ExhaustCardAction(card));
        assert_eq!(g.hand.len(), 4);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.hand.len(), 3);
        let card = g.hand.pop().unwrap();
        assert_eq!(g.hand.len(), 2);
        g.run_action(ExhaustCardAction(card));
        assert_eq!(g.hand.len(), 4);
    }

    #[test]
    fn test_brutality() {
        // check that unupgraded brutality is not innate
        for _ in 0..50 {
            let g = GameBuilder::default()
                .add_card(CardClass::Brutality)
                .add_cards(CardClass::Strike, 1000)
                .build_combat();
            if g.hand.iter().all(|c| c.borrow().class == CardClass::Strike) {
                return;
            }
        }
        panic!();
    }
}
