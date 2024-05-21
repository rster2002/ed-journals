use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchDroneEvent {
    #[serde(rename = "Type")]
    pub kind: LaunchDroneEventType,
}

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
