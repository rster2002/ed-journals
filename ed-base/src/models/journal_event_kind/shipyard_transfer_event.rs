use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardTransferEvent {
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    pub system: String,

    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,
    pub distance: f32,
    pub transfer_price: u64,
    pub transfer_time: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}
