use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplaySalaryEvent {
    pub power: String,
    pub amount: u64,
}
