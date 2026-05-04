//! Fired when the player creates a new suit load-out.

use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Suit, SuitMod, SuitModule};

/// Fired when the player creates a new suit load-out.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CreateSuitLoadoutEvent {
    /// The id of the base suit used for the load-out.
    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    /// The suit used as the base suit for the load-out.
    pub suit_name: Suit,

    /// The localized name of the base suit for the load-out.
    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: String,

    /// A list of suit mods that are part of the base suit for the load-out.
    pub suit_mods: Vec<SuitMod>,

    /// The id for the new suit load-out.
    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    /// The name for the new suit load-out.
    pub loadout_name: String,

    /// A list of equipped modules for this suit load-out.
    pub modules: Vec<SuitModule>,
}
