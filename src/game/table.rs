
use super::player::{Player, ComputerPlayer, HumanPlayer, Command};
use super::card::{Card, Suit, Hand};

use rand::{thread_rng, Rng};
use std::rc::Rc;
use std::cmp::Ordering;
use std::process::exit;

pub struct Table {
    players: Vec<Player>,
    active_players: Vec<Player>,
    deck: Vec<Rc<Card>>,
    community_cards: Vec<Rc<Card>>,
    pot: i32,
    pub largest_bet: i32,
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
        Table { players: players, active_players: Vec::new(), deck: deck, 
                community_cards: Vec::new(), pot: 0, largest_bet: 0 }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn deal_card(&mut self) -> Rc<Card> {
        let mut rng = thread_rng();
        let idx = rng.gen_range(0, self.deck.len());
        self.deck.remove(idx)
    }

    pub fn deal_cards(&mut self) {
        while !self.players.is_empty() {
            let mut player = self.players.pop().unwrap();
            let (c1, c2) = (self.deal_card(), self.deal_card());
            player.cards = Some((c1.clone(), c2.clone()));
            
            if player.is_human {
                println!("Here are your cards: [{}, {}]", c1, c2);
            }
            self.active_players.push(player);
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

    pub fn show_cards(&self) {
        for i in 0..self.active_players.len() {
            let ref player = self.active_players[i];
            let (c1, c2) = player.get_cards();
            print!("{}: {}", player.name, display_cards(&vec![c1, c2]))
        }
    }

    pub fn allow_betting(&mut self) {
        for i in 0..self.active_players.len() {
            let mut player = self.active_players.remove(i);
            let cmd = if player.is_human {
                HumanPlayer::act(&player, self)
            } else {
                ComputerPlayer::act(&player, self)
            };
            if cmd == Command::Fold {
                self.players.push(player);
            } else {
                self.process_command(cmd, &mut player);
                self.active_players.insert(i, player);
            } 
        }
    }

    pub fn process_command(&mut self, cmd: Command, player: &mut Player) {
        let largest_bet = self.largest_bet;
        match cmd {
            Command::PostBlind => self.place_bet(player, 10),
            Command::Fold => {},
            Command::Check => {},
            Command::Call => self.place_bet(player, largest_bet),
            Command::Raise(x) => {
                self.largest_bet += x;
                self.place_bet(player, largest_bet + x);
            },
            Command::Leave => exit(1),
        }

    }

    pub fn place_bet(&mut self, player: &mut Player, amount: i32) {
        player.chips -= amount;
        self.pot += amount;
    }

    pub fn evaluate_round(&mut self) {
        let mut winners = Vec::new();
        if self.active_players.len() as i32 == 1 { // one person left
            let winner = &self.active_players[0];
            let (c1, c2) = winner.get_cards();
            let mut hand = vec![c1, c2];
            hand.extend_from_slice(&self.community_cards);
            let hand = Hand::make_hand(hand).unwrap();
            
            winners.push((hand, winner.name.clone()));

        } else { // contested
            let mut hands = Vec::new();
            for player in &self.active_players {
                let mut hand = Vec::new();
                let (c1, c2) = player.get_cards();
                hand.extend_from_slice(&self.community_cards);
                hand.extend_from_slice(&[c1, c2]);
                hands.push((Hand::make_hand(hand).unwrap(), player.name.clone()));
            }

            hands.sort_by(|h1, h2| (*h2).0.cmp(&h1.0)); // sort by hand
            let (best, winner) = hands.remove(0);
            for (hand, player) in hands {
                if hand.cmp(&best) == Ordering::Equal {
                    winners.push((hand, player));
                }
            }
            winners.push((best, winner));
        };
        self.reset_table();
        self.declare_winner(winners);
    }

    fn declare_winner(&self, winners: Vec<(Hand, String)>) {
        print!("The winner(s) is : ");
        for winner in winners {
            let (hand, name) = winner;
            print!("{} with {}", name, hand);
        }
        println!("");
    }

    fn reset_table(&mut self) {
        while !self.active_players.is_empty() {
            let mut player = self.active_players.remove(0);

            let (c1, c2) = player.get_cards();
            self.deck.extend_from_slice(&[c1, c2]); // return cards to deck
            player.cards = None;

            self.players.push(player); // return player to players
        }
    }

    pub fn get_betting_round(&self) -> i32 {
        match self.community_cards.len() as i32 {
            0 => 1, // pre-flop
            3 => 2, // pre-turn
            4 => 3, // pre-river
            5 => 4, // post-river
            _ => -1, // ~impossible
        }
    }

    pub fn is_playing(&self) -> bool {
        return self.active_players.len() as i32 >= 2 && self.community_cards.len() as i32 != 5;
    }

    pub fn is_game_over(&self) -> bool {
        return self.players.len() as i32 == 1;
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
