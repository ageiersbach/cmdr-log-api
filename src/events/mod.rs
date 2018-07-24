use chrono::prelude::*;
use serde_json::*;
use std::io;

pub mod limpets;
pub mod missions;
pub mod location;
pub mod cargo;

pub use self::limpets::*;
pub use self::missions::*;
pub use self::location::*;
pub use self::cargo::*;
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerJournalEvent {
    pub event: String,
    pub timestamp: DateTime<Utc>,

    #[serde(skip)]
    pub raw_json: String,
}

impl PlayerJournalEvent {
    pub fn new(s: String) -> Result<PlayerJournalEvent> {
        let mut pje: PlayerJournalEvent = from_str(&s).unwrap();
        pje.raw_json = s.clone();
        Ok(pje)
    }
}

// missing MarketBuy?
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_journal_event_buy_drones_succeeds() {
        let test_drone = "{ \"timestamp\": \"2018-07-12T01:56:34Z\", \"event\": \"BuyDrones\", \"Type\": \"Drones\", \"Count\": 10, \"BuyPrice\": 101, \"TotalCost\": 1010 }";
        let pje = PlayerJournalEvent::new(test_drone.clone().to_string()).unwrap();
        assert_eq!(pje.raw_json, test_drone.to_string());
        let json: Value = json!(pje);
        assert_eq!(json["timestamp"], "2018-07-12T01:56:34Z");
        assert_eq!(json["event"], "BuyDrones");
    }

    #[test]
    fn player_journal_event_launch_drone_succeeds() {
        let test_drone = "{ \"timestamp\": \"2018-07-12T02:13:09Z\", \"event\": \"LaunchDrone\", \"Type\": \"Collection\" }";
        let pje = PlayerJournalEvent::new(test_drone.to_string()).unwrap();
        let json: Value = json!(pje);
        assert_eq!(json["timestamp"], "2018-07-12T02:13:09Z");
        assert_eq!(json["event"], "LaunchDrone");
    }

    #[test]
    fn player_journal_event_date() {
        let test_drone = "{ \"timestamp\": \"2018-07-12T02:13:09Z\", \"event\": \"LaunchDrone\", \"Type\": \"Collection\" }";
        let pje = PlayerJournalEvent::new(test_drone.to_string()).unwrap();
        assert_eq!(pje.date().unwrap().year(), 2018);
        assert_eq!(pje.date().unwrap().month(), 7);
        assert_eq!(pje.date().unwrap().day(), 12);
        let other_date = "2018-07-13T02:13:09Z".parse::<DateTime<Utc>>().unwrap();
        assert!(other_date > pje.date().unwrap());
        let other_date = "2018-07-11T02:13:09Z".parse::<DateTime<Utc>>().unwrap();
        assert!(other_date < pje.date().unwrap());
    }
}
