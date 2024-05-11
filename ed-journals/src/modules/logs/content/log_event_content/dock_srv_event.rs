use crate::modules::shared::ship::srv_type::SRVType;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockSRVEvent {
    #[serde(rename = "SRVType")]
    pub srv_type: SRVType,

    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localized: String,

    #[serde(rename = "ID")]
    pub id: u8,
}
