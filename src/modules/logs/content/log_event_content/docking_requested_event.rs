use serde::{Deserialize, Serialize};

use crate::modules::station::StationType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequestedEvent {
    pub station_name: String,
    pub station_type: StationType,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    #[serde(default)]
    pub landing_pads: DockingRequestedEventLandingPads,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequestedEventLandingPads {
    pub small: u8,
    pub medium: u8,
    pub large: u8,
}
