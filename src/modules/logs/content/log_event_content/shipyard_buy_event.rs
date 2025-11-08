use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipType;

/// Fired when the player buys a new ship.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEvent {
    /// The ship type for the newly bought ship.
    pub ship_type: ShipType,

    /// The localized ship type for the newly bought ship.
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,

    /// The amount of credits the player has spent on the new ship.
    pub ship_price: u64,

    /// What the player did with their current ship when buying the new ship.
    #[serde(flatten)]
    pub old_ship_action: ShipyardBuyEventOldShipAction,

    /// The current market id the action was performed. This is also where the current ship will be
    /// stored if the player choose to store their current ship.
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

/// The action that was performed for the player's current ship.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShipyardBuyEventOldShipAction {
    /// Stored the player's current ship at the current location.
    Store(ShipyardBuyEventStoreCurrentShip),

    /// Sold the player's current ship.
    Sell(ShipyardBuyEventSellCurrentShip),
}

/// Stored the player's current ship at the current location.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEventStoreCurrentShip {
    /// The ship type of the stored ship.
    pub store_old_ship: ShipType,

    /// The id of the stored ship.
    #[serde(rename = "StoreShipID")]
    pub store_ship_id: u64,
}

/// Sold the player's current ship.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEventSellCurrentShip {
    /// The ship type of the sold ship.
    pub sell_old_ship: ShipType,

    /// The id of the sold ship.
    #[serde(rename = "SellShipID")]
    pub sell_ship_id: u64,

    /// The amount of credits that the ship was sold for.
    pub sell_price: u64,
}
