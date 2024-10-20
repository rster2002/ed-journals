use serde::{Deserialize, Serialize};

use crate::modules::ship::SRVType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockSRVEvent {
    #[serde(rename = "SRVType", default)]
    pub srv_type: SRVType,

    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localized: Option<String>,

    #[serde(rename = "ID")]
    pub id: u8,
}
