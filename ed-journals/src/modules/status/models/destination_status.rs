use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DestinationStatus {
    pub system: u64,
    pub body: u8,
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
}
