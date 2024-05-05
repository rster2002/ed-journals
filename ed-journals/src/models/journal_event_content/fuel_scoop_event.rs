use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FuelScoopEvent {
    pub scooped: f32,
    pub total: f32,
}
