//! Fired when the player completes an interdiction.

use serde::{Deserialize, Serialize};

use crate::modules::commander::CombatRank;

/// Fired when the player completes an interdiction.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InterdictionEvent {
    /// Whether the interdiction was successful.
    pub success: bool,

    /// Whether the target is a player.
    pub is_player: bool,

    // TODO not sure if this is ever filled
    pub interdicted: Option<String>,
    pub combat_rank: Option<CombatRank>,

    pub faction: Option<String>,
    pub power: Option<String>,
}
