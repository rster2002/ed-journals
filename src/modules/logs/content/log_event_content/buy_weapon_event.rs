//! Fired when the player buys an (Odyssey) weapon.

use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Weapon, WeaponMod};

/// Fired when the player buys an (Odyssey) weapon.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyWeaponEvent {
    /// The weapon the player bought.
    pub name: Weapon,

    /// The localized name of the weapon the player bought.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The class of the weapon bought.
    pub class: u8,

    /// The cost in credits the player paid.
    pub price: u64,

    /// The id of the weapon.
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,

    /// List of (pre-installed) weapons mods.
    pub weapon_mods: Vec<WeaponMod>,
}
