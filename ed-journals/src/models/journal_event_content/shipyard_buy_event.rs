use crate::models::journal_event_content::shared::ship::ship_type::ShipType;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEvent {
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,
    pub ship_price: u64,

    #[serde(flatten)]
    pub old_ship_action: ShipyardBuyEventOldShipAction,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShipyardBuyEventOldShipAction {
    Store(ShipyardBuyEventStoreCurrentShip),
    Sell(ShipyardBuyEventSellCurrentShip),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEventStoreCurrentShip {
    pub store_old_ship: ShipType,

    #[serde(rename = "StoreShipID")]
    pub store_ship_id: u8,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEventSellCurrentShip {
    pub sell_old_ship: ShipType,

    #[serde(rename = "SellShipID")]
    pub sell_ship_id: u8,
    pub sell_price: u64,
}