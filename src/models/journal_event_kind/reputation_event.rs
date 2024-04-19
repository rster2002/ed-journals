use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReputationEvent {
    pub empire: f32,
    pub federation: f32,
    pub independent: f32,
    pub alliance: f32,
}
