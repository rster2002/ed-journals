use crate::modules::shared::odyssey::suit::Suit;
use crate::modules::shared::odyssey::suit_slot::SuitSlot;
use crate::modules::shared::odyssey::weapon::Weapon;
use crate::modules::shared::odyssey::weapon_mod::WeaponMod;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutRemoveModuleEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_name: Suit,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: Option<String>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: String,
    pub slot_name: SuitSlot,
    pub module_name: Weapon,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localized: Option<String>,
    pub class: u8,
    pub weapon_mods: Vec<WeaponMod>,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
}
