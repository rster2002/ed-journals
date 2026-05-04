//! Fired when the backpack.json file is updated.

use serde::{Deserialize, Serialize};

use crate::modules::odyssey::Item;

/// Fired when the backpack.json file is updated.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEvent {
    /// The items the player currently has in their backpack.
    pub items: Vec<BackpackEventObject>,

    /// The components the player currently has in their backpack.
    pub components: Vec<BackpackEventObject>,

    /// The consumables the player currently has in their backpack.
    pub consumables: Vec<BackpackEventObject>,

    /// The data the player currently has in their backpack.
    pub data: Vec<BackpackEventObject>,
}

/// An item in the player's backpack.
// TODO this is the same as ship_locker_event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEventObject {
    /// The item that is stormed.
    pub name: Item,

    /// The localized name of the item.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The ID of the owner of the item.
    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    /// ID for the mission the item is related to, if any.
    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,

    /// The number of this kind of item.
    pub count: u16,
}
