use crate::models::journal_event_kind::shared::odyssey::weapon::Weapon;
use crate::models::journal_event_kind::shared::odyssey::weapon_mod::WeaponMod;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct BuyWeaponEvent {
    // TODO replace with enum
    pub name: Weapon,

    #[serde(rename = "Name_Localised")]
    pub name_localized: String,
    pub class: u8,
    pub price: u64,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub weapon_mods: Vec<WeaponMod>,
}
