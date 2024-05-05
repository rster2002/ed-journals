use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyExplorationDataEvent {
    pub system: String,
    pub cost: u64,
}
