use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayMeritsEvent {
    pub power: String,

    pub total_merits: u32,
    pub merits_gained: u16,
}
