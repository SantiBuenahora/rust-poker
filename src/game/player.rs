use super::card::{Card, Hand};
use super::table::Table;
use ui;

use std::rc::Rc;
use std::cell::RefCell;

const CHIPS_AT_START: i32 = 100;

pub struct Player {
    pub name: String,
    pub is_human: bool,
    pub chips: i32,
    pub chips_in_play: Option<i32>,
    pub cards: Option<(Rc<Card>, Rc<Card>)>,
}

impl Player {
    pub fn new(name: String, is_human: bool) -> Player {
        Player {
            name: name,
            is_human: is_human,
            chips: CHIPS_AT_START,
            chips_in_play: None,
            cards: None,
        }
    }

    pub fn get_cards(&self) -> (Rc<Card>, Rc<Card>) {
        self.cards.as_ref().unwrap().clone()
    }

    fn get_options(table: &Table) -> Vec<Command> {
        
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
        // let options = vec![Command::Check, Command::Fold];
        // let cmd = ui::get_player_action(options);
        // table.process_command(cmd)
        Command::Check
    }
}

pub enum Command {
    PostBlind,
    Fold,
    Check,
    Call,
    Raise(i32),
    Leave,
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