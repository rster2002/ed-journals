use serde::{Serialize, Deserialize};

use crate::modules::models::odyssey::item::Item;
use crate::modules::models::odyssey::item_type::ItemType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UseConsumableEvent {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "Type")]
    pub kind: ItemType,
}
