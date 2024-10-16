use serde::{Deserialize, Serialize};

use crate::modules::ship::SRVType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockSRVEvent {
    #[serde(rename = "SRVType", default)]
    pub srv_type: SRVType,

    #[serde(rename = "SRVType_Localised", default = "default_srv_type_localized")]
    pub srv_type_localized: String,

    #[serde(rename = "ID")]
    pub id: u8,
}

fn default_srv_type_localized() -> String {
    SRVType::default().to_string()
}
