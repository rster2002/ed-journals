use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct USSDropEvent {
    #[serde(rename = "USSType")]
    pub uss_type: String,

    #[serde(rename = "USSType_Localised")]
    pub uss_type_localized: String,

    #[serde(rename = "USSThreat")]
    pub uss_threat: u8,
}
