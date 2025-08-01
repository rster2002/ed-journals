use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Weapon, WeaponMod};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct SellWeaponEvent {
    pub name: Weapon,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub class: u8,
    pub price: u64,

    #[serde(default)]
    pub weapon_mods: Vec<WeaponMod>,
}
