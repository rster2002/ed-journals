use serde::{Serialize, Deserialize};
use crate::modules::station::MissionType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionFailedEvent {
    pub name: MissionType,

    #[serde(rename = "LocalisedName")]
    pub localized_name: Option<String>,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,
    pub fine: Option<u64>,
}
