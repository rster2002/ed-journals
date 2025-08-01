use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Item, Suit};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct UpgradeSuitEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub name: Suit,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub class: u8,
    pub cost: u64,
    pub resources: Vec<UpgradeSuitEventResource>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct UpgradeSuitEventResource {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub count: u16,
}
