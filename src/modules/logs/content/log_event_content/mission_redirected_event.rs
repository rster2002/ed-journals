use serde::{Deserialize, Serialize};

use crate::modules::station::MissionType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct MissionRedirectedEvent {
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
    pub name: MissionType,
    pub localised_name: Option<String>,
    pub new_destination_station: String,
    pub new_destination_system: String,
    pub old_destination_station: String,
    pub old_destination_system: String,
}
