use crate::modules::models::trading::commodity::Commodity;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SearchAndRescueEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub name: Commodity,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub count: u16,
    pub reward: u64,
}
