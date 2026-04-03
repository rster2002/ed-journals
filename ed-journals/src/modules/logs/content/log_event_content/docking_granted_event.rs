//! Fired when the player has been given permission to dock at a given station.

use serde::{Deserialize, Serialize};

use crate::modules::station::StationType;

/// Fired when the player has been given permission to dock at a given station.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockingGrantedEvent {
    /// The station the player has been given permission to dock at.
    pub station_name: String,

    /// The kind of station the player is docking at.
    pub station_type: StationType,

    /// The market id of the station.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The landing pad the player has to dock at.
    pub landing_pad: u8,
}
