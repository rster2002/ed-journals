//! Fired when the player escapes an interdiction attempt.

use serde::{Deserialize, Serialize};

/// Fired when the player escapes an interdiction attempt.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct EscapeInterdictionEvent {
    /// The name of the player or vessel that tried to interdict the player.
    pub interdictor: String,

    /// Whether the interdictor is another player.
    #[serde(default)]
    pub is_player: bool,

    /// Whether the interdictor is a Thargoid vessel.
    #[serde(default)]
    pub is_thargoid: bool,
}

impl EscapeInterdictionEvent {
    /// Whether the interdictor is another player.
    pub fn is_player(&self) -> bool {
        self.is_player
    }

    /// Whether the interdictor is a Thargoid vessel.
    pub fn is_thargoid(&self) -> bool {
        self.is_thargoid
    }

    /// Whether the interdictor is an NPC pilot.
    pub fn is_npc_pilot(&self) -> bool {
        !self.is_thargoid && !self.is_player
    }
}
