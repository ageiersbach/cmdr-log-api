// use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::fs;
use chrono::prelude::*;
use std::env;
use std::ffi::OsStr;

use super::events::PlayerJournalEvent;
use super::api::Payload;

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
  events: Vec<PlayerJournalEvent>,
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

   pub fn api_payload_from(&self, date_time: &DateTime<Utc>) -> Option<Payload> {
     let mut payload = Payload::new(&date_time);
     for player_event in &self.events {
         if &player_event.timestamp > &date_time {
               let event_type = &player_event.event.as_str() as &str;
               match event_type {
                   "BuyDrones" => payload.buy_drones(&player_event),
                   "LaunchDrone" => payload.launch_drone(),
                   "MissionAccepted" => payload.add_mission(&player_event),
                   "MissionAbandoned" | "MissionFailed" => payload.update_mission(&player_event),
                   "Location" | "Docked" | "FSDJump" => payload.add_location(&player_event),
                   "MarketSell" => payload.add_cargo(&player_event),
                   "MiningRefined" => payload.add_single_cargo(&player_event),
                   _ => ()
               }
         }

     }
     println!("What is my drone count after all {:?}", payload.limpet_count);
     println!("what is my payload? {:?}", payload.post());
     // Some(payload)
     None
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
