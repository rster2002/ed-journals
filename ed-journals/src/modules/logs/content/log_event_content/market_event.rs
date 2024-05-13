use serde::{Serialize, Deserialize};

use crate::modules::shared::station::station_type::StationType;

/// Market data is written to a separate file called `market.json`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MarketEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub station_type: StationType,
    pub star_system: String,
}
