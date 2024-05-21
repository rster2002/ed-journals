use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RefuelPartialEvent {
    pub cost: u64,
    pub amount: f32,
}
