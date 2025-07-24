use crate::{
    actions::gain_status::GainStatusAction,
    card::CardPlayInfo,
    game::{CreatureRef, Game},
    status::Status,
};

pub fn inflame_behavior(game: &mut Game, _: Option<CreatureRef>, info: CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Strength,
        target: CreatureRef::player(),
        amount: if info.upgraded { 3 } else { 2 },
    });
}

pub fn brutality_behavior(game: &mut Game, _: Option<CreatureRef>, _: CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Brutality,
        target: CreatureRef::player(),
        amount: 1,
    });
}

#[cfg(test)]
mod tests {
    use crate::{
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
