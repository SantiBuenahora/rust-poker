use std::fmt;
use std::collections::{HashMap, BTreeMap};
use std::rc::Rc;
use std::mem;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", *self))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub val: i32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", display_val(self.val), self.suit)
    }
}

fn display_val(val: i32) -> String {
    match val {
        11 => "Jack".to_string(),
        12 => "Queen".to_string(),
        13 => "King".to_string(),
        14 => "Ace".to_string(),
        v => v.to_string(),
    }
}

#[derive(Debug)]
pub enum Hand_Category {
    High_Card,
    Pair,
    Two_Pair,
    Three_of_a_Kind,
    Straight,
    Flush,
    Full_House,
    Four_of_a_Kind,
    Straight_Flush,
}

impl fmt::Display for Hand_Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", *self).replace("_", " "))
    }
}

// invariant : hand composed of exactly 5 cards
#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Rc<Card>>,
    pub category: Hand_Category, 
}

impl Hand {
    pub fn make_hand(mut cards: Vec<Rc<Card>>) -> Result<Hand, ()> {
        if (cards.len() as i32) < 5 {
            return Err(())
        }

        // organize cards by suits
        cards.sort_by_key(|k| k.val);
        let mut suits = HashMap::new();
        let mut vals = BTreeMap::new();
        
        for card in &cards {
            let ref suit = card.suit;
            let ref val = card.val;

            if !suits.contains_key(suit) {
                suits.insert(*suit, Vec::new());
            }
            if !vals.contains_key(val) {
                vals.insert(*val, Vec::new());
            }
            let mut suit_vec = suits.remove(suit).unwrap();
            let mut val_vec = vals.remove(val).unwrap();
            suit_vec.push(card.clone());
            val_vec.push(card.clone());
            suits.insert(*suit, suit_vec);
            vals.insert(*val, val_vec);
        }

        // get existing card hands
        let mut flush = None;
        for suit in &suits {
            if (suit.1.len() as i32) >= 5 {
                flush = Some(suit);
            }
        }

        let mut straight = None;
        let mut distinct_vals = Vec::from_iter(vals.keys().map(|k| *k));
        if flush.is_some() {
            distinct_vals = Vec::from_iter(flush.unwrap().1.iter().map(|c| c.val));
        }
        // allow ace to be both 1 and 14
        if distinct_vals[0] == 2 && distinct_vals[distinct_vals.len() - 1] == 14 {
            distinct_vals.insert(0, 1);
        }
        if (distinct_vals.len() as i32) >= 5 {
            let mut range = None;
            for i in 0..(distinct_vals.len() - 4) {
                if distinct_vals[i] == distinct_vals[i + 4] - 4 {
                    if range.is_none() {
                        range = Some((distinct_vals[i], distinct_vals[i + 4]));
                    }
                    range = Some((range.unwrap().0, distinct_vals[i + 4]));
                }
            }
            if range.is_some() {
                let (start, end) = range.unwrap();
                let mut sequence = Vec::new();
                for mut i in start..(end + 1) {
                    if i == 1 { i = 14; } // ace
                    let mut card = vals[&i][0].clone();
                    if flush.is_some() {
                        for c in &vals[&i] {
                            if c.suit == *flush.unwrap().0 {
                                card = c.clone();
                            }
                        }
                    }
                    sequence.push(card);
                }
                straight = Some(sequence);
            }
        }

        let mut quad = None;
        let mut trip = None;
        let mut pairs = Vec::new();
        for val in &vals {
            let count = val.1.len();
            if count == 4 {
                quad = Some(val);
            } else if count == 3 {
                trip = Some(val);
            } else if count == 2 {
                pairs.push(val);
            }
        }

        // label hand by catagory
        let mut hand : Vec<Rc<Card>> = Vec::new();
        let mut category = Hand_Category::High_Card;
        let mut seen = Vec::new();

        if straight.is_some() && flush.is_some() {
            hand.extend_from_slice(&straight.unwrap());
            category = Hand_Category::Straight_Flush;
        
        } else if quad.is_some() {
            hand.extend_from_slice(quad.unwrap().1);
            seen.push(*quad.unwrap().0);
            category = Hand_Category::Four_of_a_Kind;

        } else if trip.is_some() && !pairs.is_empty() {
            hand.extend_from_slice(trip.unwrap().1);
            hand.extend_from_slice(pairs[pairs.len()-1].1);
            category = Hand_Category::Full_House;

        } else if flush.is_some() {
            hand.extend_from_slice(flush.unwrap().1);
            category = Hand_Category::Flush;

        } else if straight.is_some() {
            hand.extend_from_slice(&straight.unwrap());
            category = Hand_Category::Straight;

        } else if trip.is_some() {
            hand.extend_from_slice(trip.unwrap().1);
            seen.push(*trip.unwrap().0);
            category = Hand_Category::Three_of_a_Kind;            

        } else if pairs.len() as i32 >= 2 {
            hand.extend_from_slice(pairs[pairs.len()-1].1);
            hand.extend_from_slice(pairs[pairs.len()-2].1);
            seen.extend_from_slice(&[*pairs[pairs.len()-1].0, *pairs[pairs.len()-2].0]);
            category = Hand_Category::Two_Pair;

        } else if pairs.len() as i32 == 1 {
            hand.extend_from_slice(pairs[0].1);
            seen.push(*pairs[0].0);
            category = Hand_Category::Pair;

        } else {
            cards.truncate(5);
            hand.extend_from_slice(&cards);
        }

        for card in &cards {
            if (hand.len() as i32) >= 5 { break; }
            let mut has_val = false;
            for val in &seen {
                has_val = card.val == *val || has_val;
            }
            if !has_val {
                hand.push(card.clone());
            }
        }
        Ok(Hand {cards: hand, category: category })
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = format!("{} - [", self.category);
        for i in 0..5 {
            if i != 4 {
                str = str + &format!("{}, ", self.cards[i]);
            } else {
                str = str + &format!("{}]\n", self.cards[i]);
            }
        }
        write!(f, "{}", str)
    }
}