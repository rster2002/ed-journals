use crate::modules::models::ship::ship_type::ShipType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClearImpoundEvent {
    pub ship_type: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}
