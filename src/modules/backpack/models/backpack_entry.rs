use serde::Deserialize;
use crate::models::odyssey::item::Item;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEntry {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,
    pub count: u16,
}
