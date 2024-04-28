use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct BuyExplorationDataEvent {
    pub system: String,
    pub cost: u64,
}
