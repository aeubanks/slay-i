use crate::{
    actions::{gain_panache::GainPanacheAction, gain_status::GainStatusAction},
    card::CardPlayInfo,
    game::{CreatureRef, Game},
    status::Status,
};

pub fn inflame_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Strength,
        target: CreatureRef::player(),
        amount: if info.upgraded { 3 } else { 2 },
    });
}

pub fn feel_no_pain_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::FeelNoPain,
        target: CreatureRef::player(),
        amount: if info.upgraded { 4 } else { 3 },
    });
}

pub fn dark_embrace_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::DarkEmbrace,
        target: CreatureRef::player(),
        amount: 1,
    });
}

pub fn evolve_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Evolve,
        target: CreatureRef::player(),
        amount: if info.upgraded { 2 } else { 1 },
    });
}

pub fn metallicize_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Metallicize,
        target: CreatureRef::player(),
        amount: if info.upgraded { 4 } else { 3 },
    });
}

pub fn combust_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::CombustHPLoss,
        target: CreatureRef::player(),
        amount: 1,
    });
    game.action_queue.push_bot(GainStatusAction {
        status: Status::CombustDamage,
        target: CreatureRef::player(),
        amount: if info.upgraded { 7 } else { 5 },
    });
}

pub fn firebreathing_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::FireBreathing,
        target: CreatureRef::player(),
        amount: if info.upgraded { 10 } else { 6 },
    });
}

pub fn rupture_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Rupture,
        target: CreatureRef::player(),
        amount: if info.upgraded { 2 } else { 1 },
    });
}

pub fn barricade_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Barricade,
        target: CreatureRef::player(),
        amount: 1,
    });
}

pub fn brutality_behavior(game: &mut Game, _: &CardPlayInfo) {
    game.action_queue.push_bot(GainStatusAction {
        status: Status::Brutality,
        target: CreatureRef::player(),
        amount: 1,
    });
}

pub fn panache_behavior(game: &mut Game, info: &CardPlayInfo) {
    game.action_queue.push_bot(GainPanacheAction {
        amount: if info.upgraded { 14 } else { 10 },
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{block::BlockAction, draw::DrawAction, exhaust_card::ExhaustCardAction},
        cards::CardClass,
        game::{GameBuilder, Move},
        status::Status,
    };

    #[test]
    fn test_inflame() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::Inflame, None);
        assert_eq!(g.player.creature.statuses.get(&Status::Strength), Some(&2));
    }

    #[test]
    fn test_feel_no_pain() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 10)
            .build_combat();
        g.play_card(CardClass::FeelNoPain, None);
        let card = g.hand.pop().unwrap();
        g.run_action(ExhaustCardAction(card));
        assert_eq!(g.player.creature.block, 3);
        g.play_card(CardClass::FeelNoPain, None);
        let card = g.hand.pop().unwrap();
        g.run_action(ExhaustCardAction(card));
        assert_eq!(g.player.creature.block, 9);
    }

    #[test]
    fn test_dark_embrace() {
        let mut g = GameBuilder::default()
            .add_cards_upgraded(CardClass::Strike, 10)
            .build_combat();
        assert_eq!(g.hand.len(), 5);
        g.play_card_upgraded(CardClass::DarkEmbrace, None);
        let card = g.hand.pop().unwrap();
        assert_eq!(g.hand.len(), 4);
        g.run_action(ExhaustCardAction(card));
        assert_eq!(g.hand.len(), 5);
        g.play_card(CardClass::DarkEmbrace, None);
        assert_eq!(g.hand.len(), 5);
        let card = g.hand.pop().unwrap();
        assert_eq!(g.hand.len(), 4);
        g.run_action(ExhaustCardAction(card));
        assert_eq!(g.hand.len(), 6);
    }

    #[test]
    fn test_dark_embrace_ethereal() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Dazed, 20)
            .build_combat();
        assert_eq!(g.hand.len(), 5);
        g.play_card(CardClass::DarkEmbrace, None);
        g.make_move(Move::EndTurn);
        assert_eq!(g.hand.len(), 10);
    }

    #[test]
    fn test_evolve() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::Evolve, None);
        for _ in 0..10 {
            g.add_card_to_draw_pile(CardClass::Strike);
        }
        g.add_card_to_draw_pile(CardClass::Wound);
        g.run_action(DrawAction(1));
        assert_eq!(g.hand.len(), 2);
        g.play_card_upgraded(CardClass::Evolve, None);
        g.add_card_to_draw_pile(CardClass::Dazed);
        g.run_action(DrawAction(2));
        assert_eq!(g.hand.len(), 7);
    }

    #[test]
    fn test_firebreathing() {
        let mut g = GameBuilder::default().build_combat();
        let hp = g.monsters[0].creature.cur_hp;

        g.play_card(CardClass::FireBreathing, None);
        g.add_card_to_draw_pile(CardClass::Wound);
        g.run_action(DrawAction(1));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6);

        g.play_card_upgraded(CardClass::FireBreathing, None);
        g.add_card_to_draw_pile(CardClass::Dazed);
        g.add_card_to_draw_pile(CardClass::Strike);
        g.run_action(DrawAction(2));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6 - 16);
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

    #[test]
    fn test_barricade() {
        let mut g = GameBuilder::default().build_combat();
        g.play_card(CardClass::Barricade, None);
        g.run_action(BlockAction::player_flat_amount(5));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 5);
    }

    #[test]
    fn test_panache() {
        let mut g = GameBuilder::default().build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::Panache, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);
        // 5 -> 4
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);
        // 4 -> 3
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);
        // 3 -> 2
        g.play_card_upgraded(CardClass::Panache, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);
        // 2 -> 1
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp);
        // 1 -> 5
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 24);
        // 5 -> 4
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 24);
        // 4 -> 3
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 24);
        // reset to 5
        g.make_move(Move::EndTurn);
        // 5 -> 4
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 24);
        // 4 -> 3
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 24);
        // 3 -> 2
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 24);
        // 2 -> 1
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 24);
        // 1 -> 5
        g.play_card(CardClass::TestSkill, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 48);
    }
}
