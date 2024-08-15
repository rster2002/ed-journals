use crate::journal::JournalEventKind;

#[derive(Debug, Clone)]
pub struct JournalEvent {
    /// Indicates whether the event was fired from a change in the journal directory or whether
    /// it's an event from a log file other than the current one.
    pub is_live: bool,
    pub kind: JournalEventKind,
}
