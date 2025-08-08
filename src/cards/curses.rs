use crate::{
    actions::{damage::DamageAction, gain_status::GainStatusAction},
    game::{CreatureRef, Game},
    status::Status,
};

pub fn regret_behavior(game: &mut Game) {
    let hand_size = game.hand.len();
    game.action_queue.push_top(DamageAction::lose_hp(
        hand_size as i32,
        CreatureRef::player(),
    ));
}

pub fn decay_behavior(game: &mut Game) {
    game.action_queue
        .push_top(DamageAction::thorns_rupture(2, CreatureRef::player()));
}

pub fn doubt_behavior(game: &mut Game) {
    game.action_queue.push_top(GainStatusAction {
        status: Status::Weak,
        amount: 1,
        target: CreatureRef::player(),
    });
}

pub fn shame_behavior(game: &mut Game) {
    game.action_queue.push_top(GainStatusAction {
        status: Status::Frail,
        amount: 1,
        target: CreatureRef::player(),
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::block::BlockAction,
        cards::CardClass,
        game::{GameBuilder, Move},
        status::Status,
    };

    #[test]
    fn test_playable() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::AscendersBane);
        assert_eq!(g.valid_moves(), vec![Move::EndTurn]);
    }

    #[test]
    #[should_panic]
    fn test_crash_on_play_unplayable_curse() {
        let mut g = GameBuilder::default().build_combat();
        g.add_card_to_hand(CardClass::AscendersBane);
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
    }

    #[test]
    fn test_regret() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 4)
            .add_card(CardClass::Regret)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction::player_flat_amount(4));
        assert_eq!(g.player.creature.cur_hp, 50);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.cur_hp, 45);
    }

    #[test]
    fn test_regret2() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 3)
            .add_cards(CardClass::Regret, 2)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction::player_flat_amount(4));
        assert_eq!(g.player.creature.cur_hp, 50);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.cur_hp, 40);
    }

    #[test]
    fn test_decay() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Decay)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction::player_flat_amount(1));
        assert_eq!(g.player.creature.cur_hp, 50);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.cur_hp, 49);
    }

    #[test]
    fn test_doubt() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 4)
            .add_card(CardClass::Doubt)
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.statuses.get(&Status::Weak), Some(&1));
    }

    #[test]
    fn test_doubt2() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 4)
            .add_card(CardClass::Doubt)
            .add_player_status(Status::Weak, 1)
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.statuses.get(&Status::Weak), Some(&1));
    }

    #[test]
    fn test_shame() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 4)
            .add_card(CardClass::Shame)
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.statuses.get(&Status::Frail), Some(&1));
    }
}
