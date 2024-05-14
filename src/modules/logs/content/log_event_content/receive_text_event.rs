use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReceiveTextEvent {
    pub from: String,
    pub message: String,

    #[serde(rename = "Message_Localised")]
    pub message_localized: Option<String>,
    pub channel: String,
}
