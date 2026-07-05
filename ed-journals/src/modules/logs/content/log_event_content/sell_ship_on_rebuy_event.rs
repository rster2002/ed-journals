use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SellShipOnRebuyEvent {
    pub ship_type: ShipType,
    pub system: String,

    pub sell_ship_id: u64,
    pub ship_price: u64,
}
