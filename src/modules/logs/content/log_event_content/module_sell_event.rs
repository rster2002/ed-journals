use serde::{Serialize, Deserialize};

use crate::modules::models::ship::ship_module::ShipModule;
use crate::modules::models::ship::ship_slot::ShipSlot;
use crate::modules::models::ship::ship_type::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSellEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub slot: ShipSlot,
    pub sell_item: ShipModule,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localized: Option<String>,
    pub sell_price: u64,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
}
