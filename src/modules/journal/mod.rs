pub use functions::auto_detect_journal_path::auto_detect_journal_path;
pub use models::journal_event::JournalEvent;
pub use models::journal_event_kind::JournalEventKind;
pub use shared::journal_buffer::LiveJournalBufferError;

pub mod blocking;

#[cfg(feature = "asynchronous")]
#[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
pub mod asynchronous;

mod functions;
mod models;
mod shared;
