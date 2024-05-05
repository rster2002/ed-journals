use crate::models::journal_event_content::shared::odyssey::suit::Suit;
use crate::models::journal_event_content::shared::odyssey::suit_mod::SuitMod;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuySuitEvent {
    pub name: Suit,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub price: u64,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_mods: Vec<SuitMod>,
}