use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Item, ItemType};

/// Fired when the player drops an Odyssey item.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DropItemsEvent {
    /// The item the player dropped.
    pub name: Item,

    /// The localized name of the item the player dropped.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The kind of item the player dropped.
    #[serde(rename = "Type")]
    pub kind: ItemType,

    /// The owner of the dropped item.
    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    /// The number of items that were dropped.
    pub count: u16,
}
