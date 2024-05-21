use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSDiscoveryScan {
    /// Value between 0-1 to indicate percentage of system scanned
    pub progress: f32,
    pub body_count: u8,
    pub non_body_count: u8,
    pub system_name: String,
    pub system_address: u64,
}
