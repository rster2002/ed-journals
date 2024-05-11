use serde::Deserialize;

use crate::modules::shared::ship::ship_module::ShipModule;
use crate::modules::shared::ship::ship_slot::ShipSlot;
use crate::modules::shared::ship::ship_type::ShipType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleStoreEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub slot: ShipSlot,
    pub stored_item: ShipModule,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localized: String,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    pub hot: bool,
}
