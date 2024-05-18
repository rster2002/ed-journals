use serde::{Serialize, Deserialize};
use crate::modules::station::MissionType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionAbandonedEvent {
    pub name: MissionType,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,
}
