pub use functions::auto_detect_journal_path::auto_detect_journal_path;
pub use journal_event::JournalEvent;

pub mod blocking;

#[cfg(feature = "asynchronous")]
pub mod asynchronous;

mod functions;
mod journal_event;
