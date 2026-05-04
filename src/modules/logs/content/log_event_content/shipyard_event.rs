use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub star_system: String,
}
