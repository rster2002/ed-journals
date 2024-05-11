use serde::Deserialize;

use crate::modules::shared::station::station_type::StationType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockingGrantedEvent {
    pub station_name: String,
    pub station_type: StationType,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub landing_pad: u8,
}
