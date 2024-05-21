use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RefuelAllEvent {
    pub cost: u64,
    pub amount: f32,
}
