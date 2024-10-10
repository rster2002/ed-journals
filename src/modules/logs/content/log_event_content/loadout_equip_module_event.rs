use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Suit, SuitSlot, Weapon, WeaponMod};

/// Fired when equipping a module to a suit loadout.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutEquipModuleEvent {
    /// The id of the base suit used for the loadout.
    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    /// The kind of suit used for the loadout.
    pub suit_name: Suit,

    /// The localized name of the suit used for the loadout.
    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: Option<String>,

    /// The id of the loadout.
    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    /// The name of the loadout.
    pub loadout_name: String,

    /// The slot where the module was equipped.
    pub slot_name: SuitSlot,

    /// The name of the module that was equipped.
    pub module_name: Weapon,

    /// The localized module of the module that was equipped.
    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localized: Option<String>,

    /// The class of the module equipped.
    pub class: u8,

    /// A list of mods that are applied to the equipped module.
    pub weapon_mods: Vec<WeaponMod>,

    /// The id of the module.
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
}
