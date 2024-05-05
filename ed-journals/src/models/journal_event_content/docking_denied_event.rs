use crate::models::journal_event_content::shared::station::station_type::StationType;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockingDeniedEvent {
    pub station_name: String,
    pub station_type: StationType,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub reason: DockingDeniedReason,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum DockingDeniedReason {
    NoSpace,
    TooLarge,
    Hostile,
    Offences,
    Distance,
    ActiveFighter,
    NoReason,
    Other(String),
}