use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RefuelPartialEvent {
    pub cost: u64,
    pub amount: f32,
}
