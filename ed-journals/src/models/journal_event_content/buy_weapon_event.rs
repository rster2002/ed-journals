use crate::models::journal_event_content::shared::odyssey::weapon::Weapon;
use crate::models::journal_event_content::shared::odyssey::weapon_mod::WeaponMod;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
