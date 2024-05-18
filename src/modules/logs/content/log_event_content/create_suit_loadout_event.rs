use serde::{Serialize, Deserialize};
use crate::modules::odyssey::{Suit, SuitMod, SuitModule};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSuitLoadoutEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_name: Suit,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: String,
    pub suit_mods: Vec<SuitMod>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: String,
    pub modules: Vec<SuitModule>,
}
