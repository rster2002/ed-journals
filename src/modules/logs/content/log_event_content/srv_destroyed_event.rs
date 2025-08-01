use serde::{Deserialize, Serialize};

use crate::modules::ship::SRVType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct SRVDestroyedEvent {
    #[serde(rename = "ID")]
    pub id: u8,

    #[serde(rename = "SRVType")]
    pub srv_type: SRVType,

    #[serde(rename = "SRVType_Localised")]
    pub srv_localized: Option<String>,
}
