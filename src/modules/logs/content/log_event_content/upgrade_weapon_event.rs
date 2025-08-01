use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Item, Weapon};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct UpgradeWeaponEvent {
    pub name: Weapon,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub class: u8,
    pub resources: Vec<UpgradeWeaponEventResource>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct UpgradeWeaponEventResource {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub count: u16,
}
