use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayMeritsEvent {
    pub power: String,

    pub total_merits: u32,
    pub merits_gained: u16,
}
