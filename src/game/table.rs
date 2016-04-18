// use std::rand::{task_rng, Rng};

use super::player::Player;
use super::card::Card;
use super::card::Suit;

use rand::{thread_rng, Rng};
use std::rc::Rc;

const MAX_PLAYERS: i32 = 8;

pub struct Table {
    players: Vec<Rc<Player>>,
    deck: Vec<Rc<Card>>,
}

impl Table {
    pub fn build_table() -> Table {
        let mut deck = Vec::new();
        let suits = vec![Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
        for suit in suits {
            for val in 1..14 {
                deck.push(Rc::new(Card { suit: suit, val: val }));
            }
        }
        let players = Vec::new();
        Table { players: players, deck: deck }
    }

    pub fn deal_card(&mut self) -> Result<Rc<Card>, ()> {
        if (self.deck.len() as i32) != 0 {
            let mut rng = thread_rng();
            let idx = rng.gen_range(0, self.deck.len());
            Ok((self.deck.remove(idx)))
        } else {
            Err(())
        }
    }

    pub fn add_player(&mut self, player: Rc<Player>) -> Result<(), ()> {
        if (self.players.len() as i32) < MAX_PLAYERS {
            self.players.push(player);
            Ok(())
        } else {
            Err(())
        }
    }
}
