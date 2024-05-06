use serde::Deserialize;
use crate::journal_event_content::shared::odyssey::suit::Suit;
use crate::journal_event_content::shared::odyssey::suit_mod::SuitMod;

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
