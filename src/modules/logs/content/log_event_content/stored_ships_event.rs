use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipsEvent {
    pub station_name: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub star_system: String,
    pub ships_here: Vec<StoredShipEventLocalShip>,
    pub ships_remote: Vec<StoredShipEventRemoteShip>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipEventLocalShip {
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,
    pub name: Option<String>,
    pub value: u64,
    pub hot: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipEventRemoteShip {
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,
    pub name: Option<String>,

    #[serde(default)]
    pub in_transit: bool,

    #[serde(flatten)]
    pub storage_location: Option<StoredShipEventStorageLocation>,

    pub value: u64,
    pub hot: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipEventStorageLocation {
    pub star_system: String,

    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,
    pub transfer_price: u64,
    pub transfer_time: u32,
}
