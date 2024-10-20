use serde::{Deserialize, Serialize};

use crate::modules::station::{StationInfo, StationType};

/// Fired when the player docks at a station or settlement.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockedEvent {
    /// The name of the station or settlement.
    pub station_name: String,

    /// The kind of station the player docked at.
    pub station_type: StationType,

    /// The name of the star system the station is in.
    pub star_system: String,

    /// The address of the system the station is in.
    pub system_address: u64,

    /// Detailed information about the docked station.
    #[serde(flatten)]
    pub station_info: StationInfo,

    /// The distance in ls the station is from the main star.
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f32,
}
