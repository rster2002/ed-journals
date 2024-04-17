use chrono::{DateTime, Utc};
use serde::{Deserialize};
use crate::models::journal_event_kind::JournalEventKind;

#[derive(Debug, Deserialize)]
pub struct JournalEntry {
    timestamp: DateTime<Utc>,

    #[serde(flatten)]
    kind: JournalEventKind,
}
