use crate::models::journal_event_content::JournalEventContent;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// An entry from a [JournalFile]. Most of the content can be found in the [JournalEventContent].
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct JournalEvent {
    pub timestamp: DateTime<Utc>,

    #[serde(flatten)]
    pub content: JournalEventContent,
}
