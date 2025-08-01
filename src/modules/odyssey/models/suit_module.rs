use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{SuitSlot, Weapon, WeaponMod};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SuitModule {
    pub slot_name: SuitSlot,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub module_name: Weapon,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localized: String,
    pub class: u8,
    pub weapon_mods: Vec<WeaponMod>,
}
