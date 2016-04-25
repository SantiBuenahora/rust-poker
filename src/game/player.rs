use super::card::Card;
use super::table::Table;
use std::rc::Rc;

const CHIPS_AT_START: i32 = 100;

pub struct Player {
    pub name: String,
    pub is_human: bool,
    pub chips: i32,
    pub chips_in_play: Option<i32>,
    pub cards: Option<(Card, Card)>,
}

impl Player {
    pub fn new(name: String, is_human: bool) -> Player {
        Player {
            name: name,
            is_human: is_human,
            chips: CHIPS_AT_START,
            chips_in_play: None,
            cards: None
        }
    }
    pub fn set_cards(&mut self, c1: Card, c2: Card) {
        self.cards = Some((c1, c2));
    }
    pub fn print_status(&self) {
        print!("{}: ${} total; ", self.name, self.chips);
        if self.chips_in_play.is_some() {
            println!("${} in pot", self.chips_in_play.unwrap());
        } else {
            println!("Inactive in round");
        }

    }
}

pub trait ComputerPlayer {
    fn act(&mut self);
}

pub trait HumanPlayer {
    fn act(&mut self);
}

impl ComputerPlayer for Player {
    fn act(&mut self) {
        // TODO
    }
}

impl HumanPlayer for Player {
    fn act(&mut self) {
        // TODO
    }
}

    // // Execute the given command on the player and board state.
    // pub fn act(&mut self, cmd: Command) -> Result<(), ()> {
    //     match cmd {
    //         Command::PostBlind => {},
    //         Command::Fold => {},
    //         Command::Check => {},
    //         Command::Call => {},
    //         Command::Raise(i32) => {},
    //         Command::Leave => {},
    //     }
    //     Ok(())
    // }

// impl Player for ComputerPlayer {
//     // TODO
// }

// impl T for Player {

// }

// pub enum Command {
//     PostBlind,
//     Fold,
//     Check,
//     Call,
//     Raise(i32),
//     Leave,
// }

// impl Player {


//     // pub fn print_status() -> () {
//     //     println!("{}", )
//     // }
// }
