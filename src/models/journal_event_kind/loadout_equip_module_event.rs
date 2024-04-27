use serde::Deserialize;
use crate::models::journal_event_kind::shared::odyssey::suit::Suit;
use crate::models::journal_event_kind::shared::odyssey::weapon::Weapon;
use crate::models::journal_event_kind::shared::odyssey::suit_slot::SuitSlot;
use crate::models::journal_event_kind::shared::odyssey::weapon_mod::WeaponMod;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutEquipModuleEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    pub suit_name: Suit,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: String,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub slot_name: SuitSlot,
    pub module_name: Weapon,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localized: String,
    pub class: u8,
    pub weapon_mods: Vec<WeaponMod>,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
}
