use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::market::models::market_entry::MarketEntry;
use crate::modules::station::{CarrierDockingAccess, StationType};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Market {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "event")]
    pub event: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub station_type: StationType,
    pub carrier_docking_access: Option<CarrierDockingAccess>,
    pub star_system: String,
    pub items: Vec<MarketEntry>,
}
