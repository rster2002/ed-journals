use crate::modules::shared::odyssey::weapon::Weapon;
use crate::modules::shared::odyssey::weapon_mod::WeaponMod;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
