use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ResurrectEvent {
    pub option: ResurrectEventOption,
    pub cost: u64,
    pub bankrupt: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ResurrectEventOption {
    #[serde(rename = "recover")]
    Recover,

    #[serde(rename = "handin")]
    HandIn,

    #[serde(rename = "rebuy")]
    Rebuy,

    #[serde(rename = "rejoin")]
    Rejoin,

    #[serde(rename = "escape")]
    Escape,
}
