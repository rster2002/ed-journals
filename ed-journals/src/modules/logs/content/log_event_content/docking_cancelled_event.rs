use serde::Deserialize;

use crate::modules::shared::station::station_type::StationType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockingCancelled {
    pub station_name: String,
    pub station_type: StationType,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}
