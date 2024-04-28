use serde::Deserialize;
use crate::models::journal_event_kind::shared::ship::ship_module::ShipModule;
use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ModuleStoreEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    // TODO maybe replace this with an enum or a struct
    pub slot: String,

    // TODO maybe replace this with an enum or a struct
    pub stored_item: ShipModule,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localised: String,

    // TODO maybe replace this with an enum
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    pub hot: bool,
}
