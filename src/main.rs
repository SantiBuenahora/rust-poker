extern crate rand;

pub mod game;

use game::table::Table;
use game::player::Player;
use game::card::{Card, Suit, Hand};
use std::rc::Rc;

const NUM_PLAYERS: i32 = 4;

fn main() {
    let mut table = Table::build_table();
    
    let human_player = Rc::new(Player::new("Santi".to_string()));
    table.add_player(human_player.clone()).unwrap();

    let mut cpu_players = Vec::new();

    for i in 1..NUM_PLAYERS {
        let cpu_player = Rc::new(Player::new(format!("CPU_{}", i)));
        table.add_player(cpu_player.clone()).unwrap();
        cpu_players.push(cpu_player);
    }

    // make sure all cards good
    let mut cards = Vec::new();
    cards.push(Rc::new(Card { suit: Suit::Hearts, val: 5 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 5 }));
    cards.push(Rc::new(Card { suit: Suit::Spades, val: 5 }));
    cards.push(Rc::new(Card { suit: Suit::Diamonds, val: 7 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 10 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 3 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 12 }));
    
    print!("Cards: [");
    for i in 0..cards.len() {
        if i != cards.len()-1 {
            print!("{}, ", cards[i]);
        } else {
            print!("{}]\n", cards[i]);
        }
    }

    let hand = Hand::make_hand(cards).unwrap();
    print!("Hand: {} - [", hand.category);
    for i in 0..5 {
        if i != 4 {
            print!("{}, ", hand.cards[i]);
        } else {
            print!("{}]\n", hand.cards[i]);
        }
    }

    // let deal2 = table.deal_card().unwrap;
    // while dealt.is_ok() {
    //    println!("{}", dealt.unwrap()); 
    //    dealt = table.deal_card();
    // }
}
