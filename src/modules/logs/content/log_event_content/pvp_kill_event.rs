use serde::{Deserialize, Serialize};

use crate::modules::commander::CombatRank;

/// Fired when the player kills another player in combat.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PVPKillEvent {
    /// The combat rank of the killed player.
    pub combat_rank: CombatRank,

    /// The name of the killed player.
    pub victim: String,
}
