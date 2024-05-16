use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::backpack::models::backpack_entry::BackpackEntry;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Backpack {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "event")]
    pub event: String,
    pub items: Vec<BackpackEntry>,
    pub components: Vec<BackpackEntry>,
    pub consumables: Vec<BackpackEntry>,
    pub data: Vec<BackpackEntry>,
}
