use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchDroneEvent {
    #[serde(rename = "Type")]
    pub kind: LaunchDroneEventType,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
