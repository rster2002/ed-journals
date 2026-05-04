use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct SAAScanCompleteEvent {
    pub body_name: String,
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u8,

    // TODO this is not the actual data, but the wiki states that this may exist
    #[serde(default)]
    pub discoverers: Vec<String>,

    // TODO this is not the actual data, but the wiki states that this may exist
    #[serde(default)]
    pub mappers: Vec<String>,

    pub probes_used: u8,
    pub efficiency_target: u8,
}
