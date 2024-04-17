pub mod file_header_event;
mod commander_event;

use serde::{Deserialize, Serialize};
use crate::models::journal_event_kind::commander_event::CommanderEvent;
use crate::models::journal_event_kind::file_header_event::FileHeaderEvent;

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub enum JournalEventKind {
    #[serde(rename = "Fileheader")]
    FileHeader(FileHeaderEvent),

    Commander(CommanderEvent),
}
