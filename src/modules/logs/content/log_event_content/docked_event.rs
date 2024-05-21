use serde::{Deserialize, Serialize};

use crate::modules::station::{StationInfo, StationType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockedEvent {
    pub station_name: String,
    pub station_type: StationType,
    pub star_system: String,
    pub system_address: u64,

    #[serde(flatten)]
    pub station_info: StationInfo,

    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f32,
}
