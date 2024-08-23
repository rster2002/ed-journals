//! Fired when in a multi-crew session and the current player changes their role.

use serde::{Deserialize, Serialize};

/// Fired when in a multi-crew session and the current player changes their role.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeCrewRoleEvent {
    // TODO what is this?
    #[serde(default)]
    pub telepresence: bool,

    /// The new role the current player has changed to.
    pub role: ChangeCrewRoleEventRole,
}

/// The new role the current player has changed to.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChangeCrewRoleEventRole {
    /// The player has gone idle.
    Idle,

    /// The player has changed to fire controls.
    FireCon,

    /// The player has changed to ship launched fighter controls.
    FighterCon,

    /// The player has exited the ship and is currently on-foot.
    OnFoot,

    /// The player has taken control of the helm.
    Helm,
}
