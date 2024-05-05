use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RepairDroneEvent {
    pub hull_repaired: f32,
}
