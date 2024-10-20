use serde::{Deserialize, Serialize};

/// Fired when the player fires a limpet.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchDroneEvent {
    /// The kind of limpet fired.
    #[serde(rename = "Type")]
    pub kind: LaunchDroneEventType,
}

/// The kind of limpet fired.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LaunchDroneEventType {
    Hatchbreaker,
    FuelTransfer,
    Collection,
    Prospector,
    Repair,
    Research,
    Decontamination,
}
