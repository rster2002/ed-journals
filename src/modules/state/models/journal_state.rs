use serde::Serialize;
use crate::cargo::Cargo;
use crate::journal::{JournalEvent, JournalEventKind};
use crate::nav_route::NavRoute;
use crate::state::GameState;
use crate::status::Status;

#[derive(Serialize)]
pub struct JournalState {
    pub game_state: GameState,
    pub status: Option<Status>,
    pub cargo: Option<Cargo>,
    pub nav_route: Option<NavRoute>,
}

impl JournalState {
    pub fn feed_journal_event(&mut self, event: &JournalEvent) {
        match &event.kind {
            JournalEventKind::LogEvent(log_event) => {
                self.game_state.feed_log_event(log_event);
            }
            JournalEventKind::StatusEvent(status) => {
                self.status = Some(status.clone());
            }
            JournalEventKind::OutfittingEvent(_) => {}
            JournalEventKind::ShipyardEvent(_) => {}
            JournalEventKind::MarketEvent(_) => {}
            JournalEventKind::NavRoute(nav_route) => {
                self.nav_route = Some(nav_route.clone());
            }
            JournalEventKind::ModulesInfo(_) => {}
            JournalEventKind::Backpack(_) => {}
            JournalEventKind::Cargo(_) => {}
            JournalEventKind::ShipLocker(_) => {}
        }
    }

    pub fn flush(&mut self) {
        self.game_state.flush();
    }
}
