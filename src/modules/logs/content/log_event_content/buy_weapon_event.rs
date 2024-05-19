use serde::{Serialize, Deserialize};
use crate::modules::odyssey::{Weapon, WeaponMod};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyWeaponEvent {
    pub name: Weapon,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub class: u8,
    pub price: u64,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub weapon_mods: Vec<WeaponMod>,
}
