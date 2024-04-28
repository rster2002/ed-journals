use crate::models::journal_event_kind::commit_crime_event::CommitCrimeEventType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CrimeVictimEvent {
    pub offender: String,
    pub crime_type: CommitCrimeEventType,
    pub fine: Option<u64>,
    pub bounty: Option<u64>,
}
