use serde::{Deserialize, Serialize};

/// Fired when the player leaves a body.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LeaveBodyEvent {
    /// The name of the system the body is located in.
    pub star_system: String,

    /// The address of the system the body is located in.
    pub system_address: u64,

    /// The name of the body left.
    pub body: String,

    /// The id of the body.
    #[serde(rename = "BodyID")]
    pub body_id: u8,
}
