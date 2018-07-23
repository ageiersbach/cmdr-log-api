// use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::fs;
use chrono::prelude::*;
use std::env;
use std::ffi::OsStr;

use super::events::PlayerJournalEvent;

fn file_modified_today(entry: &fs::DirEntry) -> bool {
  if let Ok(metadata) = entry.metadata() {
      if let Ok(modified) = metadata.modified() {
          let d: DateTime<Utc> = modified.into();
          let today = Utc::now();
          if (today - d).num_seconds() < 86400 {
              return true
          }
      }

  }
  false
}

fn file_is_player_log(entry: &fs::DirEntry) -> bool {
    println!("what is the path? {:?}", &entry.path());
    if let Ok(_is_file) = &entry.file_type().and_then(|f| Ok(f.is_file())) {
        if let Some(extension) = &entry.path().as_path().extension().and_then(OsStr::to_str) {
            match *extension {
                "log" => return true,
                _ => return false
            }
        }
        println!("what a drag");
    }
    false
}

pub fn todays_player_journal(filename: &str) -> Option<String> {
    let entries = fs::read_dir(filename).unwrap();
    for e in entries {
        if let Ok(entry) = e {
           println!("what is");
           if file_modified_today(&entry) && file_is_player_log(&entry) {
               //return &entry.path().to_str().and_then(|p| Some(p.to_string())) why doesn't this work???
               if let Some(path_str) = &entry.path().to_str() {
                   return Some(path_str.to_string());
               }
           }

        }
    }
    None
}

#[derive(Debug)]
pub struct PlayerJournal {
  path: String,
  pub events: Vec<PlayerJournalEvent>,
}

impl PlayerJournal {
    pub fn read(path: &str) -> io::Result<PlayerJournal> {
        let f = File::open(&path)?;
        let f = BufReader::new(f);
        let mut player_events = Vec::new();

        for line in f.lines() {
          if let Ok(Ok(pje)) = line.and_then(|p| Ok(PlayerJournalEvent::new(p))) {
              player_events.push(pje);

          }
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
