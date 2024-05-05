use serde::Deserialize;

use crate::models::journal_event_content::shared::ship::ship_module::ShipModule;
use crate::models::journal_event_content::shared::ship::ship_type::ShipType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSellRemoteEvent {
    pub storage_slot: u8,
    pub sell_item: ShipModule,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localized: Option<String>,
    pub server_id: u64,
    pub sell_price: u64,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
}
