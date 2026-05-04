//! Fired whenever another player changes their own role.

use serde::{Deserialize, Serialize};

/// Fired whenever another player changes their own role.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberRoleChangeEvent {
    /// The CMDR name of the player that changed their role.
    pub crew: String,

    #[serde(default)]
    pub telepresence: bool,

    /// The new role of the player.
    pub role: CrewMemberRoleChangeEventRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum CrewMemberRoleChangeEventRole {
    Idle,
    FireCon,
    FighterCon,
    OnFoot,
    Helm,
}
