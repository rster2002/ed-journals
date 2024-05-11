use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayFastTrackEvent {
    pub power: String,
    pub cost: u64,
}
