use serde::Serialize;
use crate::backpack::Backpack;
use crate::cargo::Cargo;
use crate::journal::{JournalEvent, JournalEventKind};
use crate::logs::LogEventContent;
use crate::market::Market;
use crate::modules_info::ModulesInfo;
use crate::nav_route::NavRoute;
use crate::outfitting::Outfitting;
use crate::ship_locker::ShipLocker;
use crate::shipyard::Shipyard;
use crate::state::GameState;
use crate::status::Status;

#[derive(Serialize)]
pub struct JournalState {
    pub game_state: GameState,
    pub status: Option<Status>,
    pub modules_info: Option<ModulesInfo>,
    pub cargo: Option<Cargo>,
    pub nav_route: Option<NavRoute>,
    pub outfitting: Option<Outfitting>,
    pub shipyard: Option<Shipyard>,
    pub market: Option<Market>,
    pub backpack: Option<Backpack>,
    pub ship_locker: Option<ShipLocker>,
}

impl JournalState {
    pub fn feed_journal_event(&mut self, event: &JournalEvent) {
        match &event.kind {
            JournalEventKind::LogEvent(log_event) => {
                match &log_event.content {
                    LogEventContent::NavRouteClear => {
                        if self.nav_route
                            .as_ref()
                            .is_some_and(|nav_route| nav_route.timestamp < log_event.timestamp)
                        {
                            self.nav_route = None;
                        }
                    },
                    LogEventContent::Liftoff(_) => {
                        if self.outfitting
                            .as_ref()
                            .is_some_and(|outfitting| outfitting.timestamp < log_event.timestamp)
                        {
                            self.outfitting = None;
                        }

                        if self.shipyard
                            .as_ref()
                            .is_some_and(|shipyard| shipyard.timestamp < log_event.timestamp)
                        {
                            self.shipyard = None;
                        }

                        if self.market
                            .as_ref()
                            .is_some_and(|shipyard| shipyard.timestamp < log_event.timestamp)
                        {
                            self.market = None;
                        }
                    },
                    _ => {},
                }

                self.game_state.feed_log_event(log_event);
            }
            JournalEventKind::StatusEvent(status) => {
                self.status = Some(status.clone());
            }
            JournalEventKind::OutfittingEvent(outfitting) => {
                self.outfitting = Some(outfitting.clone());
            }
            JournalEventKind::ShipyardEvent(shipyard) => {
                self.shipyard = Some(shipyard.clone());
            }
            JournalEventKind::MarketEvent(market) => {
                self.market = Some(market.clone());
            }
            JournalEventKind::NavRoute(nav_route) => {
                self.nav_route = Some(nav_route.clone());
            }
            JournalEventKind::ModulesInfo(modules_info) => {
                self.modules_info = Some(modules_info.clone());
            }
            JournalEventKind::Backpack(backpack) => {
                self.backpack = Some(backpack.clone());
            }
            JournalEventKind::Cargo(cargo) => {
                self.cargo = Some(cargo.clone());
            }
            JournalEventKind::ShipLocker(ship_locker) => {
                self.ship_locker = Some(ship_locker.clone());
            }
        }
    }

    pub fn flush(&mut self) {
        self.game_state.flush();
    }
}
