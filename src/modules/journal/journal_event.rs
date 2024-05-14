use crate::logs::content::LogEvent;
use crate::modules::outfitting::Outfitting;
use crate::status::Status;

#[derive(Debug, PartialEq)]
pub enum JournalEvent {
    LogEvent(LogEvent),
    StatusEvent(Status),
    OutfittingEvent(Outfitting),
}
