use serde::{Deserialize, Serialize};

/// Fired when the player is fuel scooping.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FuelScoopEvent {
    /// The number of tonnes scooped.
    pub scooped: f32,

    /// The total number of tonnes in the player's fuel tank.
    pub total: f32,
}
