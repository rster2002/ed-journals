use serde::{Serialize, Deserialize};
use crate::modules::ship::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEvent {
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,
    pub ship_price: u64,

    #[serde(flatten)]
    pub old_ship_action: ShipyardBuyEventOldShipAction,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShipyardBuyEventOldShipAction {
    Store(ShipyardBuyEventStoreCurrentShip),
    Sell(ShipyardBuyEventSellCurrentShip),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEventStoreCurrentShip {
    pub store_old_ship: ShipType,

    #[serde(rename = "StoreShipID")]
    pub store_ship_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEventSellCurrentShip {
    pub sell_old_ship: ShipType,

    #[serde(rename = "SellShipID")]
    pub sell_ship_id: u64,
    pub sell_price: u64,
}
