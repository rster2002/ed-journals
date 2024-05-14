use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SellExplorationDataEvent {
    pub systems: Vec<String>,
    pub discovered: Vec<String>,
    pub base_value: u64,
    pub bonus: u64,
    pub total_earnings: u64,
}
