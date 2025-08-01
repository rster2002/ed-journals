use serde::{Deserialize, Serialize};

use crate::modules::station::StationType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct UndockedEvent {
    pub station_name: String,
    pub station_type: StationType,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    #[serde(default)]
    pub taxi: bool,
    #[serde(default)]
    pub multicrew: bool,
}
