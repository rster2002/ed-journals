use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ReceiveTextEvent {
    pub from: String,
    pub message: String,

    #[serde(rename = "Message_Localised")]
    pub message_localised: String,
    pub channel: String,
}
