mod action;
mod actions;
mod blessings;
mod card;
mod cards;
mod creature;
mod game;
mod map;
mod monster;
mod monsters;
mod move_history;
mod potion;
mod queue;
mod relic;
mod rng;
mod state;
mod status;
mod test;

use game::Game;

use crate::{
    cards::CardClass,
    creature::Creature,
    game::{CreatureRef, GameBuilder, GameStatus, Move},
    monsters::jawworm::JawWorm,
    relic::RelicClass,
};

fn creature_str(c: &Creature) -> String {
    let mut s = format!("{}: {}/{}, {} block", c.name, c.cur_hp, c.max_hp, c.block);
    if c.has_any_status() {
        let mut first = true;
        for (status, amount) in c.all_statuses() {
            if first {
                first = false;
                s += ", statuses: ";
            } else {
                s += ", ";
            }
            s.push_str(&format!("{status:?} ({amount})"));
        }
    }
    s
}

fn monster_str(c: CreatureRef, game: &Game) -> String {
    let mut i = game.monsters[c.monster_index()].behavior.get_intent();
    i.modify_damage(c, game);
    format!("{}, intent: {:?}", creature_str(game.get_creature(c)), i)
}

fn print_state(g: &Game) {
    println!("{}", creature_str(&g.player));
    println!("relics:");
    for r in &g.relics {
        println!(" {:?}: {}", r.get_class(), r.get_value());
    }
    if g.potions.iter().any(|p| p.is_some()) {
        print!("potions:");
        for p in g.potions.iter().flatten() {
            print!(" {p:?}");
        }
        println!();
    }
    println!("energy: {}", g.energy);
    println!("monsters:");
    for m in g.get_alive_monsters() {
        println!(" {}", monster_str(m, g));
    }
    println!("hand:");
    for c in &g.hand {
        println!(" {:?}", c.borrow());
    }
    println!("draw pile:");
    for c in &g.draw_pile {
        println!(" {:?}", c.borrow());
    }
    println!("discard pile:");
    for c in &g.discard_pile {
        println!(" {:?}", c.borrow());
    }
    println!("exhaust pile:");
    for c in &g.exhaust_pile {
        println!(" {:?}", c.borrow());
    }
    if let Some(c) = &g.cur_card {
        println!("current card being played: {:?}", c.borrow());
    }
    match g.result() {
        GameStatus::ExhaustCardsInHand {
            num_cards_remaining,
        } => println!("exhaust cards in hand: {num_cards_remaining} cards left"),
        GameStatus::Memories {
            num_cards_remaining,
        } => println!("memories: {num_cards_remaining} cards left"),
        _ => {}
    }
    println!("moves:");
    for (mi, m) in g.valid_moves().iter().enumerate() {
        print!(" {mi}: ");
        match m {
            Move::ChooseBlessing(b) => {
                print!("choose blessing {b:?}");
            }
            Move::Transform { card_index } => {
                print!("transform {:?}", g.master_deck[*card_index].borrow());
            }
            Move::EndTurn => print!("end turn"),
            Move::PlayCard {
                card_index: i,
                target: t,
            } => {
                print!("play card {} ({:?})", i, g.hand[*i].borrow());
                if let Some(t) = t {
                    print!(
                        " on monster {} ({})",
                        t,
                        monster_str(CreatureRef::monster(*t), g)
                    );
                }
            }
            Move::Armaments { card_index } => {
                print!(
                    "upgrade card {} ({:?})",
                    card_index,
                    g.hand[*card_index].borrow()
                );
            }
            Move::PlaceCardInHandOnTopOfDraw { card_index } => {
                print!(
                    "place card on top of draw {} ({:?})",
                    card_index,
                    g.hand[*card_index].borrow()
                );
            }
            Move::PlaceCardInDiscardOnTopOfDraw { card_index } => {
                print!(
                    "place card on top of draw {} ({:?})",
                    card_index,
                    g.discard_pile[*card_index].borrow()
                );
            }
            Move::ExhaustOneCardInHand { card_index } => {
                print!(
                    "exhaust card {} ({:?})",
                    card_index,
                    g.hand[*card_index].borrow()
                );
            }
            Move::ExhaustCardsInHand { card_index } => {
                print!(
                    "exhaust card {} ({:?})",
                    card_index,
                    g.hand[*card_index].borrow()
                );
            }
            Move::ExhaustCardsInHandEnd => {
                print!("exhaust cards end");
            }
            Move::Memories { card_index } => {
                print!(
                    "memories card {} ({:?})",
                    card_index,
                    g.discard_pile[*card_index].borrow()
                );
            }
            Move::Gamble { card_index } => {
                print!(
                    "gamble card {} ({:?})",
                    card_index,
                    g.hand[*card_index].borrow()
                );
            }
            Move::GambleEnd => {
                print!("gamble end");
            }
            Move::DualWield { card_index } => {
                print!(
                    "dual wield {} ({:?})",
                    card_index,
                    g.draw_pile[*card_index].borrow()
                );
            }
            Move::Exhume { card_index } => {
                print!(
                    "exhume card {} ({:?})",
                    card_index,
                    g.exhaust_pile[*card_index].borrow()
                );
            }
            Move::FetchCardFromDraw { card_index } => {
                print!(
                    "fetch card {} ({:?})",
                    card_index,
                    g.draw_pile[*card_index].borrow()
                );
            }
            Move::ForethoughtOne { card_index } => {
                print!(
                    "forethought one {} ({:?})",
                    card_index,
                    g.hand[*card_index].borrow()
                );
            }
            Move::ForethoughtAny { card_index } => {
                print!(
                    "forethought any {} ({:?})",
                    card_index,
                    g.hand[*card_index].borrow()
                );
            }
            Move::ForethoughtAnyEnd => {
                print!("forethought any end");
            }
            Move::Discovery { card_class } => {
                print!("discovery {:?}", card_class);
            }
            Move::DiscardPotion { potion_index } => {
                print!(
                    "discard potion {potion_index} ({:?})",
                    g.potions[*potion_index].unwrap()
                );
            }
            Move::UsePotion {
                potion_index,
                target,
            } => {
                print!(
                    "use potion {potion_index} ({:?})",
                    g.potions[*potion_index].unwrap()
                );
                if let Some(t) = target {
                    print!(
                        " on monster {} ({})",
                        t,
                        monster_str(CreatureRef::monster(*t), g)
                    );
                }
            }
        }
        println!();
    }
}

fn read_int_from_stdin(max: usize) -> usize {
    let mut s = String::new();
    loop {
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        if let Ok(v) = s.trim().parse()
            && v < max
        {
            return v;
        }
        if !s.trim().is_empty() {
            println!("invalid num \"{}\"", s.trim());
        }
    }
}

fn main() {
    let mut game = GameBuilder::default()
        .ironclad_starting_deck()
        .add_card(CardClass::Armaments)
        .add_card(CardClass::Purity)
        .add_card_upgraded(CardClass::Inflame)
        .add_relic(RelicClass::BurningBlood)
        .add_monster(JawWorm::new())
        .build();
    game.map.print();
    loop {
        match game.result() {
            GameStatus::Defeat => {
                println!("defeat :(");
                break;
            }
            GameStatus::Victory => {
                println!("victory! :)");
                break;
            }
            GameStatus::Combat
            | GameStatus::ExhaustCardsInHand { .. }
            | GameStatus::Memories { .. } => {
                print_state(&game);
                let valid_moves = game.valid_moves();
                let i = read_int_from_stdin(valid_moves.len());
                game.make_move(valid_moves[i]);
                println!("-----------------------------");
            }
        }
    }
}
