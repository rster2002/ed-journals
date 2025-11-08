use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct SellExplorationDataEvent {
    pub systems: Vec<String>,
    pub discovered: Vec<String>,
    pub base_value: u64,
    pub bonus: u64,
    pub total_earnings: u64,
}
