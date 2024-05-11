use serde::Deserialize;
use crate::logs::content::log_event_content::commit_crime_event::CommitCrimeEventType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrimeVictimEvent {
    pub offender: String,
    pub crime_type: CommitCrimeEventType,
    pub fine: Option<u64>,
    pub bounty: Option<u64>,
}
