use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReputationEvent {
    pub empire: Option<f32>,
    pub federation: Option<f32>,
    pub independent: Option<f32>,
    pub alliance: Option<f32>,
}
