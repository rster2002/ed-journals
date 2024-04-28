use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ModuleRetrieveEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    // TODO turn this into an enum
    pub slot: String,

    // TODO turn this into a struct
    pub retrieved_item: String,

    #[serde(rename = "RetrievedItem_Localised")]
    pub retrieved_item_localized: String,

    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    pub hot: bool,
}
