use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::models::station::carrier_docking_access::CarrierDockingAccess;
use crate::models::station::station_type::StationType;
use crate::modules::market::models::market_entry::MarketEntry;

#[derive(Debug, Deserialize, PartialEq)]
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
    pub carrier_docking_access: CarrierDockingAccess,
    pub star_system: String,
    pub items: Vec<MarketEntry>,
}
