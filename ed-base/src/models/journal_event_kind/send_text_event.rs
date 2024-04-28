use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SendTextEvent {
    pub to: String,
    pub message: String,
    pub sent: bool,
}
