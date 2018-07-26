#[macro_use]
pub extern crate serde_derive;

pub extern crate serde;
pub extern crate serde_json;

pub extern crate chrono;

pub mod events;
pub mod journal;
pub mod api;

use chrono::prelude::*;
use std::env;
use journal::{PlayerJournal, todays_player_journal};
use std::io::{self, BufRead, Write};

fn get_cli_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();
    input.to_string()
}

fn read_player_journal(path_name: String, commander_name: String, commencement: DateTime<Utc>) {
    println!("Before I can actually post, I must get credentials and api endpoint for {}", commander_name);
    if let Some(tpj) = todays_player_journal(&path_name) {
        if let Ok(pj) = PlayerJournal::read(&tpj) {
            pj.api_payload_from(&commencement).and_then(|api| Some(api.post()));
            //println!("What did we catch today? {:?}", pj);
        }
    }
}

fn main() {
    let mut commencement = Utc::now();
    if let Some(path_name) = env::args().nth(1) {
        println!("here be the Path you gave {:?}", &path_name);
        print!("commander_name: ");
        io::stdout().flush().unwrap();
        let commander_name = get_cli_string();
        println!("and then i can access it here: {:?}", commander_name);
        read_player_journal(path_name, commander_name, commencement);
    } else {
        println!("Please enter a valid path for player journals");
        print!("path: ");
        io::stdout().flush().unwrap();

        let path_name = get_cli_string();

        print!("commander_name: ");
        io::stdout().flush().unwrap();
        let commander_name = get_cli_string();

        println!("Preparing to proceed with commander {}, for path: {}", commander_name, path_name);
        read_player_journal(path_name, commander_name, commencement);
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2, 1+1);
    }

}
