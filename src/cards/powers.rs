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
}
