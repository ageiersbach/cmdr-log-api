use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    timestamp: DateTime<Utc>,

    #[serde(rename(deserialize = "StationName"), default)]
    station_name: String,

    #[serde(rename(deserialize = "StarSystem"))]
    system_name: String,

    #[serde(rename(deserialize = "StarPos"), default)]
    star_pos: (Option<f64>, Option<f64>, Option<f64>),
}

// impl Location {
//     pub fn is_after(&self, other: &Location) -> bool {
//         return &self.timestamp > &other.timestamp
//     }
// }
