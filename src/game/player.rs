// use super::card::Card

use super::card::Card;

const CHIPS_AT_START: i32 = 100;

pub enum Command {
    PostBlind,
    Fold,
    Check,
    Call,
    Raise(i32),
    Leave,
}

pub struct Player {
    pub name: String,
    pub chips: i32,
    pub chips_in_play: Option<i32>,
    pub cards: Option<Vec<Card>>
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            chips: CHIPS_AT_START,
            chips_in_play: None,
            cards: None
        }
    }

    // Execute the given command on the player and board state.
    pub fn act(&mut self, cmd: Command) -> Result<(), ()> {
        match cmd {
            Command::PostBlind => {},
            Command::Fold => {},
            Command::Check => {},
            Command::Call => {},
            Command::Raise(i32) => {},
            Command::Leave => {},
        }
        Ok(())
    }
}
