use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::ship_locker::ShipLockerEntry;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipLocker {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "event")]
    pub event: String,
    pub items: Vec<ShipLockerEntry>,
    pub components: Vec<ShipLockerEntry>,
    pub consumables: Vec<ShipLockerEntry>,
    pub data: Vec<ShipLockerEntry>,
}
