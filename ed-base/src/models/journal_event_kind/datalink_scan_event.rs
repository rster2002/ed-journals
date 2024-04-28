use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DatalinkScanEvent {
    // TODO maybe turn this into an enum
    pub message: String,

    #[serde(rename = "Message_Localised")]
    pub message_localized: String,
}
