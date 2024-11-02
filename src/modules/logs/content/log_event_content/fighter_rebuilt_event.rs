//! Fired when the ship launched fighter has been rebuild.

use serde::{Deserialize, Serialize};

use crate::modules::ship::FighterLoadout;

/// Fired when the ship launched fighter has been rebuild.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FighterRebuiltEvent {
    /// The load-out of the ship launched fighter.
    pub loadout: FighterLoadout,

    /// The id of the ship launched fighter.
    #[serde(rename = "ID")]
    pub id: u8,
}
