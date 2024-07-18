pub mod current_organic_location;

use serde::Serialize;
use crate::backpack::Backpack;
use crate::cargo::Cargo;
use crate::journal::{JournalEvent, JournalEventKind};
use crate::logs::LogEventContent;
use crate::logs::scan_organic_event::ScanOrganicEventScanType;
use crate::market::Market;
use crate::modules_info::ModulesInfo;
use crate::nav_route::NavRoute;
use crate::outfitting::Outfitting;
use crate::ship_locker::ShipLocker;
use crate::shipyard::Shipyard;
use crate::state::GameState;
use crate::state::models::journal_state::current_organic_location::CurrentOrganicLocation;
use crate::state::models::live_state::LiveState;
use crate::status::{Status, StatusContents};

/// State which tracks both log events and events that are fired when a json file updates.
#[derive(Serialize)]
pub struct JournalState {
    pub game_state: GameState,
    pub live_state: LiveState,
}

impl JournalState {
    pub fn feed_journal_event(&mut self, event: &JournalEvent) {
        self.live_state.feed_journal_event(&event);

        if let JournalEventKind::LogEvent(log_event) = &event.kind {
            self.game_state.feed_log_event(log_event);
        }
    }

    pub fn flush(&mut self) {
        self.game_state.flush();
    }
}
