use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct USSDropEvent {
    #[serde(rename = "USSType")]
    pub uss_type: String,

    #[serde(rename = "USSType_Localised")]
    pub uss_type_localised: String,

    #[serde(rename = "USSThreat")]
    pub uss_threat: u8,
}
