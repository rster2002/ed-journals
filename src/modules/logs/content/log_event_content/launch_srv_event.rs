use serde::{Serialize, Deserialize};

use crate::modules::shared::ship::srv_type::SRVType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchSRVEvent {
    #[serde(rename = "SRVType")]
    pub srv_type: SRVType,

    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localized: String,

    // TODO check if this can be replaced with an enum
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u8,
    pub player_controlled: bool,
}
