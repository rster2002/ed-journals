use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Item, ItemType};

/// Fired when there is a change in the player's (odyssey) backpack.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackChangeEvent {
    /// List of items which have been added to the player's backpack.
    #[serde(default)]
    pub added: Vec<BackpackChangeEventItem>,

    /// List of items which have been removed from the player's backpack.
    #[serde(default)]
    pub removed: Vec<BackpackChangeEventItem>,
}

/// Item which has been added or removed from the player's backpack.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackChangeEventItem {
    /// The item which has been changed.
    pub name: Item,

    /// The localized name of the item which has been changed.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The id of the owner of the item.
    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    /// The number of items which have been changed.
    pub count: u16,

    /// The type of items which has been changed.
    #[serde(rename = "Type")]
    pub kind: ItemType,
}
