use serde::{Deserialize, Serialize};

/// Fired when finishing a Detailed Surface Scan for a body.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SAAScanCompleteEvent {
    /// The name of the body that was scanned.
    pub body_name: String,

    /// The system address of the body that was scanned.
    pub system_address: u64,

    /// The ID of the body that was scanned.
    #[serde(rename = "BodyID")]
    pub body_id: u8,

    // TODO this is not the actual data, but the wiki states that this may exist
    #[serde(default)]
    pub discoverers: Vec<String>,

    // TODO this is not the actual data, but the wiki states that this may exist
    #[serde(default)]
    pub mappers: Vec<String>,

    /// The number of probes used during the scan.
    pub probes_used: u8,

    /// The max number of probes that can be used for the efficiency target.
    pub efficiency_target: u8,
}
