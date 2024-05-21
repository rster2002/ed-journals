use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseDestinationDropEvent {
    #[serde(rename = "Type")]
    pub kind: String,

    // TODO this appears to be empty if the destination is a fleet carrier
    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,
    pub threat: u8,
}
