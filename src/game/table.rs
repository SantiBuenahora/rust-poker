
use super::player::{Player, ComputerPlayer, HumanPlayer};
use super::card::{Card, Suit, Hand};

use rand::{thread_rng, Rng};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

pub struct Table {
    players: Vec<Rc<RefCell<Player>>>,
    active_players: Vec<Rc<RefCell<Player>>>,
    deck: Vec<Rc<Card>>,
    community_cards: Vec<Rc<Card>>,
    pot: i32,
}

impl Table {
    pub fn build_table() -> Table {
        let mut deck = Vec::new();
        let suits = vec![Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
        for suit in suits {
            for val in 2..15 {
                deck.push(Rc::new(Card { suit: suit, val: val }));
            }
        }
        let players = Vec::new();
        Table { players: players, active_players: Vec::new(),
                deck: deck, community_cards: Vec::new(), pot: 0 }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(Rc::new(RefCell::new(player)));
    }

    pub fn is_playing(&self) -> bool {
        return self.active_players.len() as i32 >= 2 && self.community_cards.len() as i32 != 5;
    }

    pub fn is_game_over(&self) -> bool {
        return self.players.len() as i32 == 1;
    }

    fn deal_card(&mut self) -> Rc<Card> {
        let mut rng = thread_rng();
        let idx = rng.gen_range(0, self.deck.len());
        self.deck.remove(idx)
    }

    pub fn deal_cards(&mut self) {
        for i in 0..self.players.len() {
            let cards = (self.deal_card(), self.deal_card());
            self.players[i].borrow_mut().set_cards(cards);
            self.active_players.push(self.players[i].clone());
            if self.players[i].borrow().is_human {
                let (c1, c2) = self.players[i].borrow().cards.as_ref().unwrap().clone();
                println!("Here are your cards: [{}, {}]", c1, c2);
            }
        }
    }

    pub fn reveal_cards(&mut self) {
        let mut revealed = Vec::new();
        match self.community_cards.len() as i32 {
            0 => revealed = vec![self.deal_card(), self.deal_card(), self.deal_card()], // flop
            3 => revealed = vec![self.deal_card()], // turn
            4 => revealed = vec![self.deal_card()], // river
            _ => {}
        }
        self.community_cards.extend_from_slice(&revealed);
        self.print_community_cards();
    }

    fn print_community_cards(&self) {
        println!("community cards: {}", display_cards(&self.community_cards));
    }

    pub fn allow_betting(&mut self) {
        for i in 0..self.active_players.len() {
            let player = self.active_players[i].as_ref();
            if player.borrow().is_human {
                HumanPlayer::act(*player, self);
            } else {
                ComputerPlayer::act(*player, self);
            }
        }
    }

    pub fn withdraw_player(&mut self, player: RefCell<Player>) {
        let mut idx : usize = 0;
        for i in 0..self.active_players.len() {
            if self.active_players[i].borrow().name == player.borrow().name {
                idx = i;
            }
        }
        self.active_players.remove(idx);
    }

    pub fn evaluate_round(&mut self) {
        let mut winners = Vec::new();
        if self.active_players.len() as i32 == 1 { // one person left
            winners.push(self.active_players.pop().unwrap());

        } else { // contested
            let mut hands = Vec::new();
            for player in &self.active_players {
                let (c1, c2) = (*player).borrow_mut().get_cards();
                let mut hand = Vec::new();
                println!("{}: {}", player.borrow().name, display_cards(&vec![c1.clone(), c2.clone()])); // DEL
                hand.extend_from_slice(&self.community_cards);
                hand.extend_from_slice(&[c1, c2]);
                hands.push((Hand::make_hand(hand).unwrap(), player.clone()));
            }
            hands.sort_by(|h1, h2| (*h2).0.cmp(&h1.0)); // sort by hand
            let (best, winner) = hands.remove(0);
            for hand in hands {
                let (hand, player) = hand;
                if hand.cmp(&best) == Ordering::Equal {
                    player.borrow_mut().hand = Some(hand);
                    winners.push(player.clone());
                }
            }
            winner.borrow_mut().hand = Some(best);
            winners.push(winner.clone());
        };
        self.reset_table();
        self.declare_winner(winners);
    }

    fn declare_winner(&self, winners: Vec<Rc<RefCell<Player>>>) {
        print!("The winner(s) is : ");
        for winner in &winners {
            let winner = (*winner).borrow();
            println!("{} with {}", winner.name, winner.hand.as_ref().unwrap());
        }
    }

    fn reset_table(&mut self) {
        for player in &self.active_players {
            let (c1, c2) = player.borrow_mut().get_cards();
            self.deck.extend_from_slice(&[c1, c2]);
            player.borrow_mut().cards = None;
        }
        self.active_players = Vec::new();
    }
}

fn display_cards(cards: &Vec<Rc<Card>>) -> String {
    let mut str = format!("[");
    for i in 0..cards.len() {
        if i != cards.len()-1 { 
            str = str + &format!("{}, ", cards[i]); 
        } else { 
            str = str + &format!("{}]\n", cards[i]); 
        }
    }
    str
}
