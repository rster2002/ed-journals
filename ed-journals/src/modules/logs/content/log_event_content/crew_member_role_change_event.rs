use serde::Deserialize;

/// Fired whenever another player changes their own role.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberRoleChangeEvent {
    /// The CMDR name of the player that changed their role.
    pub name: String,

    #[serde(default)]
    pub telepresence: bool,

    /// The new role of the player.
    pub role: CrewMemberRoleChangeEventRole,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum CrewMemberRoleChangeEventRole {
    Idle,
    FireCon,
    FighterCon,
}
