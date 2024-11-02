//! Fired when the player has successfully required permission to dock to a given station.

use serde::{Deserialize, Serialize};

use crate::modules::station::StationType;

/// Fired when the player has successfully required permission to dock to a given station.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequestedEvent {
    /// The name of the station the player wants to dock at.
    pub station_name: String,

    /// The kind of station the player wants to dock at.
    pub station_type: StationType,

    /// The market id of the station.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The number of available landing pads at the given station.
    pub landing_pads: DockingRequestedEventLandingPads,
}

/// The number of available landing pads at the given station.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequestedEventLandingPads {
    /// The total number of small landing pads at the given station.
    pub small: u8,

    /// The total number of medium landing pads at the given station.
    pub medium: u8,

    /// The total number of large landing pads at the given station.
    pub large: u8,
}
