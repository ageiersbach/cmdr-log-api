use super::events::*;
use serde_json::*;
use chrono::prelude::*;

#[derive(Serialize, Debug)]
pub struct Payload {
  pub limpet_count: i32,
  pub new_missions: Vec<AddMission>,
  pub updated_missions: Vec<UpdateMission>,
  pub locations: Vec<Location>,
  pub cargo: Vec<Cargo>,
}

impl Payload {
    pub fn new(timestamp: &DateTime<Utc>) -> Payload {
      Payload {
          limpet_count: 0,
          new_missions: vec![],
          updated_missions: vec![],
          locations: vec![],
          cargo: vec![],
      }
    }

    pub fn post(&self) -> String {
        to_string(&self).expect("could not serialize Payload")
        // todo: pass in the API ENDPOINT, and send https post with payload (overloading method?)
    }

    pub fn add_mission(&mut self, event: &PlayerJournalEvent) {
        let new_mission: AddMission = from_str(&event.raw_json).expect("could not parse mission");
        self.new_missions.push(new_mission);
    }

    pub fn update_mission(&mut self, event: &PlayerJournalEvent) {
        let update_mission: UpdateMission = from_str(&event.raw_json).expect("could not parse mission");
        self.updated_missions.push(update_mission);
    }

    pub fn add_location(&mut self, event: &PlayerJournalEvent) {
        let location: Location = from_str(&event.raw_json).expect("Could not parse location");
        self.locations.push(location);
    }

    pub fn add_cargo(&mut self, event: &PlayerJournalEvent) {
        let cargo: Cargo = from_str(&event.raw_json).expect("Could not parse Cargo");
        self.cargo.push(cargo);
    }

    pub fn add_single_cargo(&mut self, event: &PlayerJournalEvent) {
        let cargo_item: CargoItem = from_str(&event.raw_json).expect("Could not parse Cargo Item");
        let cargo = Cargo {
            count: 1,
            timestamp: cargo_item.timestamp,
            event: cargo_item.event,
            commodity: cargo_item.commodity
        };
        self.cargo.push(cargo);
    }

    pub fn buy_drones(&mut self, event: &PlayerJournalEvent) {
        let limpet: Limpets = from_str(&event.raw_json).unwrap();
        self.limpet_count += limpet.count;
    }

    pub fn launch_drone(&mut self) {
        self.limpet_count -= 1;
    }
}
