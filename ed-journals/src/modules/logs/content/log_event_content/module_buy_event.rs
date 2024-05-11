use serde::Deserialize;

use crate::modules::shared::ship::ship_module::ShipModule;
use crate::modules::shared::ship::ship_slot::ShipSlot;
use crate::modules::shared::ship::ship_type::ShipType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleBuyEvent {
    pub slot: ShipSlot,
    pub stored_item: Option<ShipModule>,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localized: Option<String>,
    pub buy_item: ShipModule,

    #[serde(rename = "BuyItem_Localised")]
    pub buy_item_localized: Option<String>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub buy_price: u64,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
}
