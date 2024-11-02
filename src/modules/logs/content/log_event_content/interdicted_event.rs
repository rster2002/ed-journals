//! Fired when the player has been interdicted and lost.

use serde::{Deserialize, Serialize};

/// Fired when the player has been interdicted and lost.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InterdictedEvent {
    /// Whether the player submitted to the interdiction.
    pub submitted: bool,

    // TODO check when this is [None]
    /// The name of the interdictor.
    pub interdictor: Option<String>,

    /// Whether the interdictor is a player.
    #[serde(default)]
    pub is_player: bool,

    /// Whether the interdictor is a Thargoid vessel.
    #[serde(default)]
    pub is_thargoid: bool,

    // TODO check when this is [None]
    /// The faction the interdictor is a part of.
    pub faction: Option<String>,
}

impl InterdictedEvent {
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
