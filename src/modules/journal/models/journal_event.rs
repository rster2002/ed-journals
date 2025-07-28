use crate::journal::JournalEventKind;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[deprecated]
pub struct JournalEvent {
    /// Indicates whether the event was fired from a change in the journal directory or whether
    /// it's an event from a log file other than the current one.
    pub is_live: bool,
    pub kind: JournalEventKind,
}
