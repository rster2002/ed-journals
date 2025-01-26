use serde::{Deserialize, Serialize};

use crate::modules::odyssey::Item;

/// An item stored in the player's backpack.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEntry {
    /// The item stored in the player's backpack.
    pub name: Item,

    /// Localized name of the item.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The id of the owner the item belongs to.
    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    /// The number of the same item.
    pub count: u16,
}
