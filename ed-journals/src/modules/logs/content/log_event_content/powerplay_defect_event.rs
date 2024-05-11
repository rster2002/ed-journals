use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayDefectEvent {
    pub from_power: String,
    pub to_power: String,
}
