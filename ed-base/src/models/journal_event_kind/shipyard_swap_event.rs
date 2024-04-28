use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardSwapEvent {
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    pub store_old_ship: ShipType,

    #[serde(rename = "StoreShipID")]
    pub store_ship_id: u8,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}
