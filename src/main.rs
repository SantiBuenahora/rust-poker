extern crate rand;

pub mod game;
pub mod ui;

use game::table::Table;
use game::player::Player;
use game::card::{Card, Suit, Hand};
use std::rc::Rc;

fn main() {
    let mut table = Table::build_table();
    ui::game_setup(&mut table);
    loop {
        table.deal_cards();
        while table.is_playing() {
            table.reveal_cards();
            table.allow_betting();
        } 
        table.evaluate_round(); // round over
        if table.is_game_over() { // game over
            println!("The game is over! Thank you for playing :)");
        }
    }
}

fn process_cards(cards: Vec<Rc<Card>>) -> () {
    print!("Cards: [");
    for i in 0..cards.len() {
        if i != cards.len()-1 { print!("{}, ", cards[i]); } 
        else { print!("{}]\n", cards[i]); }
    }

    let hand = Hand::make_hand(cards).unwrap();
    println!("{}\n", hand);
}

fn test_hands() -> () {
    // test : Straight
    let mut cards = Vec::new();
    cards.push(Rc::new(Card { suit: Suit::Spades, val: 8 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 14 }));
    cards.push(Rc::new(Card { suit: Suit::Hearts, val: 12 }));
    cards.push(Rc::new(Card { suit: Suit::Spades, val: 13 }));
    cards.push(Rc::new(Card { suit: Suit::Spades, val: 11 }));
    cards.push(Rc::new(Card { suit: Suit::Spades, val: 10 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 12 }));
    // process_cards(cards);

    // test : two 'three of a kind's => need 7 over 5
    let mut cards = Vec::new();
    cards.push(Rc::new(Card { suit: Suit::Hearts, val: 5 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 5 }));
    cards.push(Rc::new(Card { suit: Suit::Spades, val: 5 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 7 }));
    cards.push(Rc::new(Card { suit: Suit::Spades, val: 7 }));
    cards.push(Rc::new(Card { suit: Suit::Diamonds, val: 7 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 12 }));
    // process_cards(cards);

    // test : two 'two pair's
    let mut cards = Vec::new();
    cards.push(Rc::new(Card { suit: Suit::Hearts, val: 5 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 5 }));
    cards.push(Rc::new(Card { suit: Suit::Spades, val: 6 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 6 }));
    cards.push(Rc::new(Card { suit: Suit::Spades, val: 7 }));
    cards.push(Rc::new(Card { suit: Suit::Diamonds, val: 7 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 4 }));
    // process_cards(cards);

    // test : 'Full House'
    let mut cards = Vec::new();
    cards.push(Rc::new(Card { suit: Suit::Hearts, val: 5 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 5 }));
    cards.push(Rc::new(Card { suit: Suit::Spades, val: 6 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 6 }));
    cards.push(Rc::new(Card { suit: Suit::Diamonds, val: 6 }));
    cards.push(Rc::new(Card { suit: Suit::Diamonds, val: 7 }));
    cards.push(Rc::new(Card { suit: Suit::Clubs, val: 7 }));
    // process_cards(cards);
}