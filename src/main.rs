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

fn main() {
    if let Some(path_name) = env::args().nth(1) {
        println!("here be the Path you gave {:?}", &path_name);
        if let Some(tpj) = todays_player_journal(&path_name) {
            //let time_before_read = Utc::now();
            let time_before_read = "2018-05-10T02:13:09Z".parse::<DateTime<Utc>>().expect("could not parse date");

            if let Ok(pj) = PlayerJournal::read(&tpj) {
                pj.api_payload_from(&time_before_read).and_then(|api| Some(api.post()));
                //println!("What did we catch today? {:?}", pj);
            }
        }
    } else {
        println!("Please enter a valid path for player journals");
    }

    // loop -> read file -> parse event_type, s
    //events::hello();
    // let player_jounral = journal::get_newest_logfile_at_path();
    // let mut timestamp: DateTime = initialize_somehow(); // ????
    // loop ->
    //   player_journal.next_unread_line() ->
    //     let event = parse_json_event(line);
    //     match event.type ->
    //       ShutDown => break,
    //       API => send_http(host, event.to_http()),
    //       _ => println!('...')
    //     timestamp = event.timestamp;
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2, 1+1);
    }

}
