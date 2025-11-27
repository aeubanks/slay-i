mod action;
mod actions;
mod blessings;
mod card;
mod cards;
mod creature;
mod draw_pile;
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
mod step;
mod test;

use game::Game;

use crate::{
    cards::CardClass,
    game::{GameBuilder, GameStatus},
    monsters::jawworm::JawWorm,
    relic::RelicClass,
};

fn print_state(g: &Game) {
    println!("{}", g.player.str());
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
        println!(" {}", g.monster_str(m));
    }
    println!("hand:");
    for c in &g.hand {
        println!(" {:?}", c.borrow());
    }
    println!("draw pile:");
    for c in g.draw_pile.get_all() {
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
    for (si, s) in g.valid_steps().iter().enumerate() {
        println!(" {si}: {}", s.description(g));
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
                let valid_steps = game.valid_steps();
                let i = read_int_from_stdin(valid_steps.len());
                game.step(valid_steps.into_iter().nth(i).unwrap());
                println!("-----------------------------");
            }
        }
    }
}
