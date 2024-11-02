//! Market data is written to a separate file called `market.json`.

use serde::{Deserialize, Serialize};

use crate::modules::station::StationType;

/// Market data is written to a separate file called `market.json`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MarketEvent {
    /// The market the update is related to.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The name of the station updates are available for.
    pub station_name: String,

    /// The type of station.
    pub station_type: StationType,

    /// The name of the star system the market is located in.
    pub star_system: String,
}
