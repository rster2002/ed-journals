use crate::models::journal_event_content::shared::station::mission_type::MissionType;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionAbandonedEvent {
    pub name: MissionType,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,
}
