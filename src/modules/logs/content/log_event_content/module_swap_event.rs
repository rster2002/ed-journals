//! Fired when swapping a module from one slot to another.

use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

use crate::modules::ship::{ShipSlot, ShipType};
use crate::ship::ShipModule;

/// Fired when swapping a module from one slot to another.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSwapEvent {
    /// The market id the player is performing the action at.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The slot that the module was originally in.
    pub from_slot: ShipSlot,

    /// The slot that the module has been moved to.
    pub to_slot: ShipSlot,

    /// The module that was in the 'from' slot.
    pub from_item: ShipModule,

    /// The localized name of the module that was in the 'from' slot.
    #[serde(rename = "FromItem_Localised")]
    pub from_item_localized: Option<String>,

    /// The module that was originally in the 'to' slot and that has now been placed in the 'from'
    /// slot.
    #[serde(deserialize_with = "deserialize_to_item")]
    pub to_item: Option<ShipModule>,

    /// The localized name of the module that was in the 'to' slot.
    #[serde(rename = "ToItem_Localised")]
    pub to_item_localized: Option<String>,

    /// The id of the current active ship.
    pub ship: ShipType,

    /// Whether the module is hot.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

fn deserialize_to_item<'de, D>(deserializer: D) -> Result<Option<ShipModule>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;

    if &string == "Null" {
        Ok(None)
    } else {
        ShipModule::from_str(&string)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}
