use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardSellEvent {
    pub ship_type: ShipType,

    #[serde(rename = "SellShipID")]
    pub sell_ship_id: u64,
    pub ship_price: u64,
    pub system: Option<String>,
}
