use serde::Deserialize;
use crate::journal_event_content::shared::ship::ship_type::ShipType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SellShipOnRebuyEvent {
    pub ship_type: ShipType,
    pub system: String,

    #[serde(rename = "SellShipID")]
    pub sell_ship_id: u8,
    pub ship_price: u64,
}
