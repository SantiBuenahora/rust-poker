use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::mem;

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
        1 => "Ace".to_string(),
        11 => "Jack".to_string(),
        12 => "Queen".to_string(),
        13 => "King".to_string(),
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
        cards.sort_by_key(|k| -(if k.val == 1 { 14 } else { k.val }));
        let mut suits = HashMap::new();
        let mut vals = HashMap::new();
        
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

        // let mut straight = None;
        // let mut distinct_vals = Vec::new();
        // for val in vals.keys() {
        //     distinct_vals.push(*val);
        // }
        // if (distinct_vals.len() as i32) >= 5 {
        //     let mut start = None;
        //     let mut end = None;
        //     for i in 0..(distinct_vals.len() - 4) {
        //         if distinct_vals[i] == distinct_vals[i+4] - 4 {
        //             if start.is_none() && end.is_none() {
        //                 start = Some(i);
        //             }
        //             end = Some(i + 4);
        //         }
        //     }
        //     if start.is_some() && end.is_some() {
        //         let (start, end) = (start.unwrap(), end.unwrap());
        //         let mut sequence = Vec::new();
        //         for val in start..(end + 1) {
        //             let card = vals[&distinct_vals[val]][0].clone();
        //             sequence.push(card);
        //         }
        //         straight = Some(sequence);
        //     }
        // }

        // let mut straight_flush = None;
        // if flush.is_some() && straight.is_some() {
        //     let straight = straight.unwrap();
        //     let suit = flush.unwrap().0;
        //     'outer: for i in 0..(straight.len() - 4) {
        //         if straight_flush.is_some() {
        //             break 'outer;
        //         }
        //         let mut sequence = Vec::new();
        //         'inner: for j in i..(i + 5) {
        //             if straight[j].suit != *suit {
        //                 break 'inner;
        //             }
        //             sequence.push(straight[j].clone());
        //         }
        //         if sequence.len() as i32 == 5 {
        //             straight_flush = Some(sequence);
        //         }
        //     }
        // }

        let mut quad = None;
        let mut trip = None;
        let mut pairs = Vec::new();
        for val in &vals {
            let count = val.1.len();
            if count == 4 {
                quad = Some(val);
            } else if count == 3 {
                if trip.is_none() {
                    trip = Some(val);
                }
            } else if count == 2 {
                pairs.push(val);
            }
        }

        // label hand by catagory
        let mut included : Vec<Rc<Card>> = Vec::new();
        let mut category = Hand_Category::High_Card;
        let mut seen = Vec::new();

        /* if straight_flush.is_some() '
        []            hand = straight_flush.unwrap();
            category = Hand_Category::Straight_Flush;
        
        } else */ if quad.is_some() {
            included.extend_from_slice(quad.unwrap().1);
            seen.push(*quad.unwrap().0);
            category = Hand_Category::Four_of_a_Kind;

        } else if trip.is_some() && !pairs.is_empty() {
            included.extend_from_slice(trip.unwrap().1);
            included.extend_from_slice(pairs[0].1);
            category = Hand_Category::Full_House;

        } else if flush.is_some() {
            included.extend_from_slice(flush.unwrap().1);
            category = Hand_Category::Flush;

        } /* else if straight.is_some() {
            hand = straight.unwrap();
            category = Hand_Category::Straight;

        } */ else if trip.is_some() {
            included.extend_from_slice(trip.unwrap().1);
            seen.push(*trip.unwrap().0);
            category = Hand_Category::Three_of_a_Kind;            

        } else if pairs.len() as i32 >= 2 {
            included.extend_from_slice(pairs[0].1);
            included.extend_from_slice(pairs[1].1);
            seen.extend_from_slice(&[*pairs[0].0, *pairs[1].0]);
            category = Hand_Category::Two_Pair;

        } else if pairs.len() as i32 == 1 {
            included.extend_from_slice(pairs[0].1);
            seen.push(*pairs[0].0);
            category = Hand_Category::Pair;

        } else {
            cards.truncate(5);
            included.extend_from_slice(&cards);
        }

        let mut hand = Vec::new();
        for card in included {
            hand.push(card.clone());
        }
        for card in &cards {
            if (hand.len() as i32) < 5 {
                break;
            }
            let mut has_val = false;
            for val in &seen {
                if card.val == *val {
                    has_val = true;
                }
            }
            if !has_val {
                hand.push(card.clone());
            }
        }
        Ok(Hand {cards: hand, category: category })
    }
}