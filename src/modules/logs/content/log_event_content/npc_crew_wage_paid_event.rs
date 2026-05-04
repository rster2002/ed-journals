//! Fired when the player paid the wage of an NPC, which happens automatically.

use serde::{Deserialize, Serialize};

/// Fired when the player paid the wage of an NPC, which happens automatically.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct NPCCrewWagePaidEvent {
    /// The name of the NPC that was paid.
    pub npc_crew_name: String,

    /// The id of the NPC that was paid.
    pub npc_crew_id: u64,

    /// The number of credits that were paid to the NPC.
    pub amount: u64,
}
