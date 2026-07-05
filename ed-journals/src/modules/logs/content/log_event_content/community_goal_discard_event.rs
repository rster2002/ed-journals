//! Fired when the player discards a community goal.

use serde::{Deserialize, Serialize};

/// Fired when the player discards a community goal.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalDiscardEvent {
    /// The id of the community goal.
    #[serde(rename = "CGID")]
    pub cgid: u64,

    /// The name of the community goal.
    pub name: String,

    /// The system where the community goal is located in.
    pub system: String,
}
