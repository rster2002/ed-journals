use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::logs::content::LogEventContent;

/// An entry from a [JournalFile]. Most of the content can be found in the [JournalEventContent].
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct LogEvent {
    pub timestamp: DateTime<Utc>,

    #[serde(flatten)]
    pub content: LogEventContent,
}
