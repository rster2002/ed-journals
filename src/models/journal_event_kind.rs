pub mod file_header_event;
mod commander_event;
mod materials_event;
mod rank_event;
mod progress_event;
mod reputation_event;
mod engineer_progress_event;
mod load_game_event;

use serde::{Deserialize, Serialize};
use crate::models::journal_event_kind::commander_event::CommanderEvent;
use crate::models::journal_event_kind::engineer_progress_event::EngineerProgressEvent;
use crate::models::journal_event_kind::file_header_event::FileHeaderEvent;
use crate::models::journal_event_kind::materials_event::MaterialsEvent;
use crate::models::journal_event_kind::progress_event::ProgressEvent;
use crate::models::journal_event_kind::rank_event::RankEvent;
use crate::models::journal_event_kind::reputation_event::ReputationEvent;

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub enum JournalEventKind {
    #[serde(rename = "Fileheader")]
    FileHeader(FileHeaderEvent),
    Commander(CommanderEvent),
    Materials(MaterialsEvent),
    Rank(RankEvent),
    Progress(ProgressEvent),
    Reputation(ReputationEvent),
    EngineerProgress(EngineerProgressEvent),
    LoadGame(),
}
