use crate::modules::shared::odyssey::suit::Suit;
use crate::modules::shared::odyssey::suit_mod::SuitMod;
use crate::modules::shared::odyssey::suit_slot::SuitSlot;
use crate::modules::shared::odyssey::weapon::Weapon;
use crate::modules::shared::odyssey::weapon_mod::WeaponMod;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SuitLoadoutEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_name: Suit,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: String,
    pub suit_mods: Vec<SuitMod>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: String,
    pub modules: Vec<SuitLoadoutModule>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SuitLoadoutModule {
    pub slot_name: SuitSlot,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub module_name: Weapon,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localized: String,
    pub class: u8,
    pub weapon_mods: Vec<WeaponMod>,
}