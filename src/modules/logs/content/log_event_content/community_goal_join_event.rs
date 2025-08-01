//! Fired when the player joins the community event.

use serde::{Deserialize, Serialize};

/// Fired when the player joins the community event.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalJoinEvent {
    /// The id of the community goal.
    #[serde(rename = "CGID")]
    pub cgid: u64,

    /// The name of the community goal.
    pub name: String,

    /// The system where the community goal is located in.
    pub system: String,
}
