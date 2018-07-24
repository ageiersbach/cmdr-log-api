#[derive(Serialize, Deserialize, Debug)]
pub struct AddMission {
  #[serde(rename(deserialize = "LocalisedName"))]
  name: String,

  #[serde(rename(deserialize = "Commodity_Localised"))] //possible to be blank?
  commodity: String,

  #[serde(rename(deserialize = "DestinationSystem"))]
  destination_system: String,

  #[serde(rename(deserialize = "DestinationStation"))]
  destination_station: String,

  #[serde(rename(deserialize = "Expiry"))]
  expires_at: String,

  #[serde(rename(deserialize = "MissionID"))]
  ed_mission_id: i32,

  #[serde(rename(deserialize = "Count"))]
  pub count: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMission {
  event: String,

  #[serde(rename(deserialize = "MissionID"))]
  ed_mission_id: i32,
}
