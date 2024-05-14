use crate::modules::models::odyssey::suit::Suit;
use crate::modules::models::odyssey::suit_mod::SuitMod;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SellSuitEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub name: Suit,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub price: u64,

    #[serde(default)]
    pub suit_mods: Vec<SuitMod>,
}
