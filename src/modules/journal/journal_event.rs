use crate::logs::content::LogEvent;
use crate::status::Status;

#[derive(Debug, PartialEq)]
pub enum JournalEvent {
    LogEvent(LogEvent),
    StatusEvent(Status),
}
