use serde::Deserialize;
use crate::models::journal_event_content::shared::galaxy::body_type::BodyType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseExitEvent {
    pub star_system: String,
    pub system_address: u64,
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub body_type: BodyType,

    #[serde(default)]
    pub taxi: bool,

    #[serde(default)]
    pub multicrew: bool,
}
