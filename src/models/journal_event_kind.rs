pub mod file_header_event;
mod commander_event;
mod load_game_event;
mod statistics_event;
mod reputation_event;
mod approach_body_event;
mod clear_saved_game_event;
mod docked_event;
mod docking_cancelled_event;
mod docking_denied_event;
mod docking_granted_event;
mod docking_requested_event;
mod docking_timeout_event;
mod engineer_progress_event;
mod fsd_jump_event;
mod loadout_event;
mod materials_event;
mod missions_event;
mod new_commander_event;
mod passengers_event;
mod powerplay_event;
mod progress_event;
mod rank_event;

use serde::{Deserialize, Serialize};
use crate::models::journal_event_kind::approach_body_event::ApproachBodyEvent;
use crate::models::journal_event_kind::clear_saved_game_event::ClearSavedGameEvent;
use crate::models::journal_event_kind::commander_event::CommanderEvent;
use crate::models::journal_event_kind::docked_event::DockedEvent;
use crate::models::journal_event_kind::docking_cancelled_event::DockingCancelled;
use crate::models::journal_event_kind::docking_denied_event::DockingDeniedEvent;
use crate::models::journal_event_kind::docking_granted_event::DockingGrantedEvent;
use crate::models::journal_event_kind::docking_requested_event::DockingRequestedEvent;
use crate::models::journal_event_kind::docking_timeout_event::DockingTimeoutEvent;
use crate::models::journal_event_kind::engineer_progress_event::EngineerProgressEvent;
use crate::models::journal_event_kind::file_header_event::FileHeaderEvent;
use crate::models::journal_event_kind::fsd_jump_event::FSDJumpEvent;
use crate::models::journal_event_kind::load_game_event::LoadGameEvent;
use crate::models::journal_event_kind::loadout_event::LoadoutEvent;
use crate::models::journal_event_kind::materials_event::MaterialsEvent;
use crate::models::journal_event_kind::missions_event::MissionsEvent;
use crate::models::journal_event_kind::new_commander_event::NewCommanderEvent;
use crate::models::journal_event_kind::passengers_event::PassengersEvent;
use crate::models::journal_event_kind::powerplay_event::PowerplayEvent;
use crate::models::journal_event_kind::progress_event::ProgressEvent;
use crate::models::journal_event_kind::rank_event::RankEvent;
use crate::models::journal_event_kind::reputation_event::ReputationEvent;
use crate::models::journal_event_kind::statistics_event::StatisticsEvent;

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub enum JournalEventKind {
    // Startup
    #[serde(rename = "Fileheader")]
    FileHeader(FileHeaderEvent),
    Commander(CommanderEvent),
    Materials(MaterialsEvent),
    Rank(RankEvent),
    Progress(ProgressEvent),
    Reputation(ReputationEvent),
    EngineerProgress(EngineerProgressEvent),
    LoadGame(LoadGameEvent),
    Powerplay(PowerplayEvent),
    Passengers(PassengersEvent),
    Statistics(StatisticsEvent),
    NewCommander(NewCommanderEvent),
    Loadout(LoadoutEvent),
    ClearSavedGame(ClearSavedGameEvent),
    Missions(MissionsEvent),

    // Travel
    ApproachBody(ApproachBodyEvent),
    Docked(DockedEvent),
    DockingCancelled(DockingCancelled),
    DockingDenied(DockingDeniedEvent),
    DockingGranted(DockingGrantedEvent),
    DockingRequested(DockingRequestedEvent),
    DockingTimeout(DockingTimeoutEvent),
    FSDJump(FSDJumpEvent),
}
