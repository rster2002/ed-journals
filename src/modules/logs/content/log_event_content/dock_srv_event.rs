use serde::{Serialize, Deserialize};
use crate::modules::ship::SRVType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockSRVEvent {
    #[serde(rename = "SRVType")]
    pub srv_type: SRVType,

    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localized: String,

    #[serde(rename = "ID")]
    pub id: u8,
}
