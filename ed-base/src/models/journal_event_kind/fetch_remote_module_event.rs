use crate::models::journal_event_kind::shared::ship::ship_module::ShipModule;
use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FetchRemoteModuleEvent {
    pub storage_slot: u8,
    pub stored_item: ShipModule,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localized: String,
    pub server_id: u64,
    pub transfer_cost: u64,
    pub transfer_time: u64,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
}
