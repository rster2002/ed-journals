use crate::modules::shared::ship::ship_type::ShipType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SellShipOnRebuyEvent {
    pub ship_type: ShipType,
    pub system: String,

    #[serde(rename = "SellShipID")]
    pub sell_ship_id: u8,
    pub ship_price: u64,
}
