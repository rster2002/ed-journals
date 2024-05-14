use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SendTextEvent {
    pub to: String,
    pub message: String,
    pub sent: bool,
}
