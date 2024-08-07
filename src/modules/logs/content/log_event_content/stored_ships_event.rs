use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipType;

/// Fired for locations that have stored ships for the player.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipsEvent {
    /// The name of the current station the ships are stored at.
    pub station_name: String,

    /// The market id of the locations where the ships are stored.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The name of the star system where the ships are stored.
    pub star_system: String,

    /// List of ships that are stored at the current location.
    pub ships_here: Vec<StoredShipEventLocalShip>,

    /// List of ships that are stored at locations other than the current one.
    pub ships_remote: Vec<StoredShipEventRemoteShip>,
}

/// Entry for a locally stored ship.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipEventLocalShip {
    /// The id of the stored ship.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    /// The ship type of the stored ship.
    pub ship_type: ShipType,

    /// The localized ship type of the stored ship.
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,

    /// The custom name for the ship, if any.
    pub name: Option<String>,

    /// The value if the ship configuration in credits.
    pub value: u64,

    /// Whether the ship is wanted.
    pub hot: bool,
}

/// Entry for a remotely stored ship.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipEventRemoteShip {
    /// The id of the ship that is stored remotely.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    /// The ship type of the remotely stored ship.
    pub ship_type: ShipType,

    /// The localized ship type of the remotely stored ship.
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,

    /// The custom name of the remotely stored ship. Unlike how [StoredShipEventLocalShip] works,
    /// if the ship does not have a custom name is an empty string.
    pub name: String,

    /// Whether the ship is currently being transferred between locations.
    #[serde(default)]
    pub in_transit: bool,

    /// The current storage location. `None` the ship is currently being transferred between
    /// locations.
    #[serde(flatten)]
    pub storage_location: Option<StoredShipEventStorageLocation>,

    /// The total value of the ship.
    pub value: u64,

    /// Whether the ship is wanted.
    pub hot: bool,
}

/// Information about the location the ship is currently stored.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipEventStorageLocation {
    /// The name of the star system the ship is currently located at.
    pub star_system: String,

    /// The market id for the location the ship is currently stored.
    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,

    /// The price in credits required to transfer the ship to the player's current location.
    pub transfer_price: u64,

    /// The time it would take to transfer the ship to the player's current location in seconds.
    pub transfer_time: u32,
}
