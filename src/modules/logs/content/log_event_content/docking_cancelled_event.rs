//! Fired when the player cancels the docking procedure.

use serde::{Deserialize, Serialize};

use crate::modules::station::StationType;

/// Fired when the player cancels the docking procedure.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockingCancelled {
    /// The name of the station or settlement the player cancelled the docking procedure for.
    pub station_name: String,

    /// The kind of station the player was going to dock at.
    pub station_type: StationType,

    /// The market id for the station.
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}
