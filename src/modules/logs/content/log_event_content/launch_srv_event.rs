use serde::{Deserialize, Serialize};

use crate::modules::ship::SRVType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchSRVEvent {
    // The default is necessary because when SRVs were released, there was only
    // the Scarab. Therefore, this field did not exist.
    #[serde(rename = "SRVType", default)]
    pub srv_type: SRVType,

    // See above
    #[serde(rename = "SRVType_Localised", default = "default_srv_type_localized")]
    pub srv_type_localized: String,

    // TODO check if this can be replaced with an enum
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u8,
    pub player_controlled: bool,
}

fn default_srv_type_localized() -> String {
    SRVType::default().to_string()
}
