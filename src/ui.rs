use std::io::{self, Write};
use std::rc::Rc;

use game::player::{Player, Command};
use game::table::Table;

pub fn game_setup(table: &mut Table) {
    println!("\nWelcome to Rust-Poker!");
    println!("======================\n");
    
    // TO DELETE
    let player_name = "Santi".to_string();
    let num_players = 5;
    let human_player = Player::new(player_name, true);
    table.add_player(human_player);

    // let mut player_name = terminal_request("What's your name?");
    // let human_player = Player::new(player_name, true);
    // table.add_player(human_player);

    // let mut num_players = 0;
    // while num_players == 0 {
    //     let mut num = 1738; // I'm like "Hey, what's up? Hello"
    //     while num == 1738 {
    //         num = terminal_request("How many players in a game (must be \
    //                             between 2 and 9)?").parse().unwrap_or(1738);
    //         if num == 1738 {
    //             println!("Invalid input! Not a number!");
    //         }
    //     }
    //     if num >= 2 && num <= 9 {
    //         num_players = num;
    //     } else {
    //         println!("Invalid input! Must be between 2 and 9!");
    //     }
    // }
    
    for i in 1..num_players {
        let cpu_player = Player::new(format!("CPU_{}", i), false);
        table.add_player(cpu_player);
    }
}

pub fn get_player_action(option: Vec<Command>) -> Command {
    let mut action = terminal_request("Would you like to <check> or <fold>?");
    match action.to_lowercase().as_str() {
        "check" => Command::Check,
        "fold" => Command::Fold,
        _ => Command::Check,
    }
}

fn terminal_request(request : &str) -> String {
    println!("{}", request);
    print!("> ");

    while true {
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {
                return buf.trim().to_string();
            }
            _ => {}
        }
        println!("Invalid input!");
        println!("Again, {}", request);
        print!("> ");
    }
    return String::new();
}
