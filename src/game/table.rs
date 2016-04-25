
use super::player::{Player, ComputerPlayer, HumanPlayer};
use super::card::{Card, Suit};

use rand::{thread_rng, Rng};
use std::rc::Rc;

pub struct Table {
    players: Vec<Rc<Player>>,
    active_players: Vec<Rc<Player>>,
    deck: Vec<Card>,
    community_cards: Vec<Card>,
    pot: i32,
}

impl Table {
    pub fn build_table() -> Table {
        let mut deck = Vec::new();
        let suits = vec![Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
        for suit in suits {
            for val in 2..15 {
                deck.push(Card { suit: suit, val: val });
            }
        }
        let players = Vec::new();
        Table { players: players, active_players: Vec::new(),
                deck: deck, community_cards: Vec::new(), pot: 0 }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(Rc::new(player));
    }

    pub fn is_playing(&self) -> bool {
        return self.active_players.len() as i32 >= 2 && self.community_cards.len() as i32 != 5;
    }

    pub fn is_game_over(&self) -> bool {
        return self.players.len() as i32 == 1;
    }

    fn deal_card(&mut self) -> Card {
        let mut rng = thread_rng();
        let idx = rng.gen_range(0, self.deck.len());
        self.deck.remove(idx)
    }

    pub fn deal_cards(&mut self) {
        let cards = (self.deal_card(), self.deal_card());
        for player in &mut self.players {
            (*player).set_cards(cards.0, cards.1);
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
    }

    pub fn allow_betting(&mut self) {
        for player in self.players {
            if player.is_human {
                HumanPlayer::act(&mut *player);
            } else {
                ComputerPlayer::act(&mut *player.clone());
            }
        }    
    }

    pub fn evaluate_round(&mut self) {
        // TODO
    }
}
