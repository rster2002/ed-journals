//! Fired when assigning crew.

use serde::{Deserialize, Serialize};

/// Fired when assigning crew.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewAssignEvent {
    /// The name of the crew that was assigned.
    pub name: String,

    /// The id of the crew member that was assigned a new role.
    #[serde(rename = "CrewID")]
    pub crew_id: u64,

    /// The new role of the crew member.
    pub role: CrewAssignEventRole,
}

/// The role of the crew member.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum CrewAssignEventRole {
    /// The crew member has been set to active.
    Active,
    /// The crew member has been set to inactive.
    OnShoreLeave,
}
