//! Fired when a ship launched fighter docks back to the mother ship.

use serde::{Deserialize, Serialize};

/// Fired when a ship launched fighter docks back to the mother ship.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct DockFighterEvent {
    /// The id of the docked fighter.
    #[serde(rename = "ID")]
    pub id: u8,
}
