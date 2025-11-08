//! Fired when deleting a suit load-out.

use serde::{Deserialize, Serialize};

use crate::modules::odyssey::Suit;

/// Fired when deleting a suit load-out.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSuitLoadoutEvent {
    /// The id of the suit that was used for the load-out.
    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    /// The suit that was used for the load-out.
    pub suit_name: Suit,

    /// The localized name of the suit that was used for the load-out.
    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: String,

    /// The id of the deleted load-out.
    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    /// The name of the deleted load-out.
    pub loadout_name: String,
}
