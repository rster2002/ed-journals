use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::modules::outfitting::models::outfitting_entry::OutfittingEntry;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Outfitting {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "event")]
    pub event: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub star_system: String,
    pub horizons: bool,
    pub items: Vec<OutfittingEntry>,
}
