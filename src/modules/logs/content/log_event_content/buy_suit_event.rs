use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Suit, SuitMod};

/// Fired when the player buys a new Odyssey suit.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuySuitEvent {
    /// The suit that the player bought.
    pub name: Suit,

    /// The localized name of the suit that was bought.
    #[serde(rename = "Name_Localised")]
    pub name_localized: String,

    /// The number of credits the player paid.
    pub price: u64,

    /// The id of the newly bought suit.
    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    /// List of (pre-installed) suit mods.
    pub suit_mods: Vec<SuitMod>,
}
