use crate::modules::shared::odyssey::suit::Suit;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSuitLoadoutEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_name: Suit,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: String,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: String,
}
