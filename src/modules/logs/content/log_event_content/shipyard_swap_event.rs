use serde::{Serialize, Deserialize};
use crate::modules::ship::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardSwapEvent {
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub store_old_ship: ShipType,

    #[serde(rename = "StoreShipID")]
    pub store_ship_id: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}
