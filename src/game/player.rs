use super::card::Card;
use super::table::Table;
use ui;

use std::rc::Rc;
use std::fmt;

const CHIPS_AT_START: i32 = 100;

pub struct Player {
    pub name: String,
    pub is_human: bool,
    pub chips: i32,
    pub chips_in_play: i32,
    pub cards: Option<(Rc<Card>, Rc<Card>)>,
}

impl Player {
    pub fn new(name: String, is_human: bool) -> Player {
        Player {
            name: name,
            is_human: is_human,
            chips: CHIPS_AT_START,
            chips_in_play: 0,
            cards: None,
        }
    }

    pub fn get_cards(&self) -> (Rc<Card>, Rc<Card>) {
        self.cards.as_ref().unwrap().clone()
    }

    fn get_options(&self, table: &Table) -> Vec<Command> {
        let largest_bet = table.largest_bet.clone();
        let chips = self.chips.clone();
        let chips_in_play = self.chips_in_play.clone();
        assert!(chips > largest_bet);

        let mut options = if table.get_betting_round() == 1 {
            vec![Command::PostBlind]
        
        } else {
            vec![Command::Check]
            // // let mut options = vec![Command::Raise(chips - largest_bet)];
            // if largest_bet > chips_in_play {
            //     options.push(Command::Call);

            // } else if largest_bet == chips_in_play {
            //     options.push(Command::Check);
            // }
            // options
        };
        options.extend_from_slice(&[Command::Fold, Command::Leave]);
        options
    }
}

pub trait ComputerPlayer {
    fn act(&self, table: &Table) -> Command;
}

pub trait HumanPlayer {
    fn act(&self, table: &Table) -> Command;
}

impl ComputerPlayer for Player {
    fn act(&self, table: &Table) -> Command {
        // TODO
        Command::Check
    }
}

impl HumanPlayer for Player {
    fn act(&self, table: &Table) -> Command {
        let options = self.get_options(table);
        ui::get_player_action(options)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    PostBlind,
    Fold,
    Check,
    Call,
    Raise(i32),
    Leave,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Command::PostBlind => write!(f, "Post Blind"),
            &Command::Raise(x) => write!(f, "Raise _ (max is {})", x),
            cmd => write!(f, "{}", format!("{:?}", cmd)),
        }
    }
}

    // // Execute the given command on the player and board state.
    // pub fn act(&mut self, cmd: Command) -> Result<(), ()> {
    //     match cmd {

    //     }
    //     Ok(())
    // }

// impl Player for ComputerPlayer {
//     // TODO
// }

// impl T for Player {

// }

// impl Player {


//     // pub fn print_status() -> () {
//     //     println!("{}", )
//     // }
// }


        // match  {
        //     Command::PostBlind => {},
        //     Command::Fold => {}, // table.withdraw_player(self.clone());
        //     Command::Check => {},
        //     Command::Call => {},
        //     Command::Raise(i32) => {},
        //     Command::Leave => {},
        // }

    // pub fn print_status(&self) {
    //     print!("{}: ${} total; ", self.name, self.chips);
    //     if self.chips_in_play.is_some() {
    //         println!("${} in pot", self.chips_in_play.unwrap());
    //     } else {
    //         println!("Inactive in round");
    //     }
    // }