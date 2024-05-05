use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReceiveTextEvent {
    pub from: String,
    pub message: String,

    #[serde(rename = "Message_Localised")]
    pub message_localised: Option<String>,
    pub channel: String,
}
