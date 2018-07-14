// use std::env;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::io::prelude::*;

use super::events::PlayerJournalEvent;

struct PlayerJournal {
  path: String,
  events: Vec<PlayerJournalEvent>,
}

impl PlayerJournal {
    fn read(path: &str) -> io::Result<PlayerJournal> {
        let f = File::open(&path)?;
        let f = BufReader::new(f);
        let mut player_events = Vec::new();

        for line in f.lines() {
          let pje = PlayerJournalEvent::new(line.unwrap());
          player_events.push(pje);
        }

        Ok(PlayerJournal {
          path: path.to_string(),
          events: player_events
       })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_clones_path_to_player_journal() {
        let mut f = File::create("foo.txt").expect("could not create file for some reason");
        assert!(f.write(b"{\"event\": \"Shutdown\", \"timestamp\": \"2018-07-22T01:53:10Z\"  }\n").is_ok());
        assert!(f.write(b"{\"event\": \"BuyDrones\", \"timestamp\": \"2018-07-22T01:57:10Z\"  }\n").is_ok());

        let pj = PlayerJournal::read("foo.txt").unwrap();
        assert_eq!(pj.path, "foo.txt");
        assert_eq!(pj.events.len(), 2);
        assert!(fs::remove_file("foo.txt").is_ok());
    }

}
