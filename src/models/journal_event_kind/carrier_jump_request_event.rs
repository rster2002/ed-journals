use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJumpRequestEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub system_name: String,

    // TODO check why this is sometimes [None]
    pub body: Option<String>,
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub departure_time: DateTime<Utc>,
}
