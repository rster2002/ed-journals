//! Fired when the player did not dock within the given time window.

use serde::{Deserialize, Serialize};

use crate::modules::station::StationType;

/// Fired when the player did not dock within the given time window.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct DockingTimeoutEvent {
    /// The name of the station the player had requested to dock at.
    pub station_name: String,

    /// The kind of station the player wanted to dock at.
    pub station_type: StationType,

    /// The market id of the station.
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}
