use crate::modules::shared::station::mission_type::MissionType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionAbandonedEvent {
    pub name: MissionType,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,
}
