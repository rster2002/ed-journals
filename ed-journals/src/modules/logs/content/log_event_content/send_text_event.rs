use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SendTextEvent {
    pub to: String,
    pub message: String,
    pub sent: bool,
}
