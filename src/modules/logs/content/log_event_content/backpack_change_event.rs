use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Item, ItemType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackChangeEvent {
    #[serde(default)]
    pub added: Vec<BackpackChangeEventItem>,

    #[serde(default)]
    pub removed: Vec<BackpackChangeEventItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackChangeEventItem {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,
    pub count: u16,

    #[serde(rename = "Type")]
    pub kind: ItemType,
}
