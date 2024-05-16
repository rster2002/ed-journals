use serde::{Serialize, Deserialize};
use crate::models::odyssey::suit_mod::SuitMod;
use crate::models::odyssey::suit_slot::SuitSlot;
use crate::models::odyssey::weapon::Weapon;
use crate::models::odyssey::weapon_mod::WeaponMod;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SwitchSuitLoadoutEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    // TODO replace this with an enum
    pub suit_name: String,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: String,
    pub suit_mods: Vec<SuitMod>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: String,
    pub modules: Vec<SwitchSuitLoadoutEventModule>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SwitchSuitLoadoutEventModule {
    pub slot_name: SuitSlot,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub module_name: Weapon,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localized: String,
    pub class: u8,
    pub weapon_mods: Vec<WeaponMod>,
}
