use serde::{Deserialize, Serialize};

use crate::modules::odyssey::Suit;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct RenameSuitLoadoutEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_name: Suit,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: Option<String>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: String,
}
