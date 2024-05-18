use serde::Deserialize;
use crate::modules::odyssey::Item;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipLockerEntry {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,
    pub count: u16,
}
