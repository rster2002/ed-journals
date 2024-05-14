use serde::{Serialize, Deserialize};

use crate::modules::shared::odyssey::suit::Suit;
use crate::modules::shared::odyssey::suit_mod::SuitMod;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuySuitEvent {
    pub name: Suit,

    #[serde(rename = "Name_Localised")]
    pub name_localized: String,
    pub price: u64,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_mods: Vec<SuitMod>,
}
