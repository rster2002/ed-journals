mod touchdown_location;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::backpack::Backpack;
use crate::cargo::Cargo;
use crate::journal::{JournalEvent, JournalEventKind};
use crate::logs::{LogEvent, LogEventContent};
use crate::market::Market;
use crate::modules_info::ModulesInfo;
use crate::nav_route::NavRoute;
use crate::outfitting::Outfitting;
use crate::ship_locker::ShipLocker;
use crate::shipyard::Shipyard;
use crate::state::models::live_state::touchdown_location::TouchdownLocation;
use crate::status::{Status, StatusContents};

/// Life state tracks state from the logs and combines them with state from live files like for
/// example the status.json file to provide more context in some instances. Something that is
/// special about this state it that is cannot be constructed from just the log files, so this is
/// the only state that can de [Serialize]d and importantly [Deserialize]d. This way state can be
/// saved to disk and retrieved at a later time when you want to continue with the same state.
#[derive(Serialize, Deserialize, Default)]
pub struct LiveState {
    pub touchdown_locations: Vec<TouchdownLocation>,

    // started_at: DateTime<Utc>,
    //
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



impl LiveState {
    pub fn feed_journal_event(&mut self, event: &JournalEvent) {
        match &event.kind {
            JournalEventKind::LogEvent(log_event) => {
                self.feed_log_event(log_event);
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

    pub fn feed_log_event(&mut self, log_event: &LogEvent) {
        match &log_event.content {
            LogEventContent::Touchdown(touchdown) => {
                let Some(status) = &self.status else {
                    return;
                };

                let StatusContents::Ship(ship_status) = &status.contents else {
                    return;
                };

                self.touchdown_locations.push(TouchdownLocation {
                    system_address: touchdown.system_address,
                    body_id: touchdown.body_id,
                    coordinates: todo!(),
                })
            },
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
    }
}
//
// impl Default for LiveState {
//     fn default() -> Self {
//         LiveState {
//             started_at: Utc::now(),
//             status: None,
//             modules_info: None,
//             cargo: None,
//             nav_route: None,
//             outfitting: None,
//             shipyard: None,
//             market: None,
//             backpack: None,
//             ship_locker: None,
//         }
//     }
// }
