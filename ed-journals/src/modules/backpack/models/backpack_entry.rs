use serde::{Deserialize, Serialize};

use crate::modules::odyssey::Item;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEntry {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,
    pub count: u16,
}
