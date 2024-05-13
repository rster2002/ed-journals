use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::logs::content::LogEventContent;

/// An entry from a [LogFile](super::super::LogFile). Most of the content can be found in the
/// [LogEventContent].
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogEvent {
    pub timestamp: DateTime<Utc>,

    #[serde(flatten)]
    pub content: LogEventContent,
}
