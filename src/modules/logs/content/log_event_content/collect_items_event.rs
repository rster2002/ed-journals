//! Fired when the player collects an Odyssey item.

use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Item, ItemCategory};

/// Fired when the player collects an Odyssey item.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CollectItemsEvent {
    /// The item the player has collected.
    pub name: Item,

    /// The localized name of the item the player has collected.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The type of item the player collected.
    #[serde(rename = "Type")]
    pub kind: ItemCategory,

    /// The id of the owner of the item.
    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    /// The number of items the player collected.
    pub count: u16,

    /// Whether the items are considered stolen.
    pub stolen: bool,
}
