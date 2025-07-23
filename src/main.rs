mod action;
mod actions;
mod blessings;
mod card;
mod cards;
mod creature;
mod game;
mod monster;
mod monsters;
mod move_history;
mod player;
mod queue;
mod relic;
mod status;

use game::Game;

use crate::{
    cards::CardClass,
    creature::Creature,
    game::{GameBuilder, GameStatus, Move},
    monster::Monster,
    monsters::jawworm::JawWorm,
    player::Player,
    relic::RelicClass,
};

fn creature_str(c: &Creature) -> String {
    let mut s = format!("{}: {}/{}, {} block", c.name, c.cur_hp, c.max_hp, c.block);
    if !c.statuses.is_empty() {
        let mut first = true;
        for (status, amount) in &c.statuses {
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

fn monster_str(m: &Monster, player: &Player) -> String {
    let mut i = m.behavior.get_intent();
    i.modify_damage(&m.creature, player);
    format!("{}, intent: {:?}", creature_str(&m.creature), i)
}

fn print_state(g: &Game) {
    println!("{}", creature_str(&g.player.creature));
    println!("energy: {}", g.energy);
    println!("monsters:");
    for m in &g.monsters {
        if !m.creature.is_alive() {
            continue;
        }
        println!(" {}", monster_str(m, &g.player));
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
    println!("moves:");
    for (mi, m) in g.valid_moves().iter().enumerate() {
        print!(" {mi}: ");
        match m {
            Move::ChooseBlessing(b) => {
                print!("choose blessing {b:?}");
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
                        monster_str(&g.monsters[*t], &g.player)
                    );
                }
            }
            Move::Armaments { card_index: i } => {
                print!("upgrade card {} ({:?})", i, g.hand[*i].borrow());
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
        if let Ok(v) = s.trim().parse() {
            if v < max {
                return v;
            }
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
        .add_card_upgraded(CardClass::Inflame)
        .add_relic(RelicClass::BurningBlood)
        .add_monster(JawWorm::new())
        .build();
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
            GameStatus::Combat | GameStatus::Armaments => {
                print_state(&game);
                let valid_moves = game.valid_moves();
                let i = read_int_from_stdin(valid_moves.len());
                game.make_move(valid_moves[i]);
                println!("-----------------------------");
            }
        }
    }
}
