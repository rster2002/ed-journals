use serde::Deserialize;

/// Fired when in a multi-crew session and the current player changes their role.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeCrewRoleEvent {
    #[serde(default)]
    pub telepresence: bool,
    pub role: ChangeCrewRoleEventRole,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum ChangeCrewRoleEventRole {
    Idle,
    FireCon,
    FighterCon,
}
