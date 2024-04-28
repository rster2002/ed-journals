use crate::models::journal_event_kind::shared::station::mission_type::MissionType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct MissionAbandonedEvent {
    pub name: MissionType,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,
}
