use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cargo {
    pub timestamp: DateTime<Utc>,
    pub event: String,

    #[serde(rename(deserialize = "Count"))]
    pub count: i32,

    #[serde(rename(deserialize = "Type"))]
    pub commodity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CargoItem {
    pub timestamp: DateTime<Utc>,
    pub event: String,

    #[serde(rename(deserialize = "Type_Localised"))]
    pub commodity: String,
}
