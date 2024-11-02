//! Fired when the player receives credits for participating in a community goal.

use serde::{Deserialize, Serialize};

/// Fired when the player receives credits for participating in a community goal.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalRewardEvent {
    /// The id of the community goal.
    #[serde(rename = "CGID")]
    pub cgid: u64,

    /// The name of the community goal.
    pub name: String,

    /// The system where the community goal is located in.
    #[serde(alias = "System")]
    pub system_name: String,

    /// The number of credits the player has been awarded for participating in the community goal.
    pub reward: u64,
}
