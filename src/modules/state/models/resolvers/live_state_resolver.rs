pub mod touchdown_location;
pub mod organic_location;

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
use crate::state::models::feed_result::FeedResult;
use crate::state::models::resolvers::live_state_resolver::organic_location::OrganicLocation;
use crate::state::models::resolvers::live_state_resolver::touchdown_location::TouchdownLocation;
use crate::state::traits::state_resolver::StateResolver;
use crate::status::{PlanetStatus, ShipStatus, Status, StatusContents};

/// Life state tracks state from the logs and combines them with state from live files like for
/// example the status.json file to provide more context in some instances. Something that is
/// special about this state it that is cannot be constructed from just the log files, so this is
/// the only state that can de [Serialize]d and importantly [Deserialize]d. This way state can be
/// saved to disk and retrieved at a later time when you want to continue with the same state.
#[derive(Serialize, Deserialize, Default)]
pub struct LiveStateResolver {
    /// The locations where the player has landed on planets.
    pub touchdown_locations: Vec<TouchdownLocation>,

    /// The locations of scanned organics on different planets.
    pub organic_locations: Vec<OrganicLocation>,

    pub status: Option<Status>,
    pub modules_info: Option<ModulesInfo>,
    pub cargo: Option<Cargo>,
    pub nav_route: Option<NavRoute>,
    pub outfitting: Option<Outfitting>,
    pub shipyard: Option<Shipyard>,
    pub market: Option<Market>,
    pub backpack: Option<Backpack>,
    pub ship_locker: Option<ShipLocker>,

    /// This field keeps track of the last known ship status. This can be used to access information
    /// about the player's ship when the player is on-foot.
    pub last_ship_status: Option<ShipStatus>,
}

impl StateResolver<JournalEvent> for LiveStateResolver {
    fn feed(&mut self, input: &JournalEvent) -> FeedResult {
        match &input.kind {
            JournalEventKind::LogEvent(log_event) => {
                self.feed_log_event(log_event);
            }
            JournalEventKind::StatusEvent(status) => {
                self.status = Some(status.clone());

                let Some(contents) = status.contents.as_ref() else {
                    return FeedResult::Skipped;
                };

                if let Some(ship_status) = contents.kind.ship_status() {
                    self.last_ship_status = Some(ship_status.clone());
                }
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

        FeedResult::Accepted
    }
}

impl LiveStateResolver {
    fn feed_log_event(&mut self, input: &LogEvent) -> FeedResult {
        match &input.content {
            LogEventContent::Touchdown(touchdown) => {
                let Some(planet_status) = self.valid_planet_status(&input.timestamp) else {
                    return FeedResult::Skipped;
                };

                self.touchdown_locations.push(TouchdownLocation {
                    system_address: touchdown.system_address,
                    body_id: touchdown.body_id,
                    coordinates: (planet_status.latitude, planet_status.latitude),
                })
            },
            LogEventContent::ScanOrganic(scan_organic) => {
                let Some(planet_status) = self.valid_planet_status(&input.timestamp) else {
                    return FeedResult::Skipped;
                };

                self.organic_locations.push(OrganicLocation {
                    system_address: scan_organic.system_address,
                    body_id: scan_organic.body,
                    species: scan_organic.species.clone(),
                    variant: scan_organic.variant.clone(),
                    coordinates: (planet_status.latitude, planet_status.longitude),
                })
            },
            LogEventContent::NavRouteClear => {
                if self.valid_nav_route(&input.timestamp).is_some() {
                    self.nav_route = None;
                }
            },
            LogEventContent::Liftoff(_) => {
                if self.valid_outfitting(&input.timestamp).is_some() {
                    self.outfitting = None;
                }

                if self.valid_shipyard(&input.timestamp).is_some() {
                    self.shipyard = None;
                }

                if self.valid_market(&input.timestamp).is_some() {
                    self.market = None;
                }
            },
            _ => {},
        }

        FeedResult::Accepted
    }

    /// Returns the valid available status based on the given timestamp.
    pub fn valid_status(&self, timestamp: &DateTime<Utc>) -> Option<&Status> {
        let status= self.status.as_ref()?;

        if timestamp >= &status.timestamp {
            Some(status)
        } else {
            None
        }
    }

    /// Returns the valid available module info based on the given timestamp.
    pub fn valid_module_info(&self, timestamp: &DateTime<Utc>) -> Option<&ModulesInfo> {
        let module_info = self.modules_info.as_ref()?;

        if timestamp >= &module_info.timestamp {
            Some(module_info)
        } else {
            None
        }
    }

    /// Returns the valid available cargo based on the given timestamp.
    pub fn valid_cargo(&self, timestamp: &DateTime<Utc>) -> Option<&Cargo> {
        let cargo = self.cargo.as_ref()?;

        if timestamp >= &cargo.timestamp {
            Some(cargo)
        } else {
            None
        }
    }

    /// Returns the valid available nav route based on the given timestamp.
    pub fn valid_nav_route(&self, timestamp: &DateTime<Utc>) -> Option<&NavRoute> {
        let nav_route = self.nav_route.as_ref()?;

        if timestamp >= &nav_route.timestamp {
            Some(nav_route)
        } else {
            None
        }
    }

    /// Returns the valid available outfitting based on the given timestamp.
    pub fn valid_outfitting(&self, timestamp: &DateTime<Utc>) -> Option<&Outfitting> {
        let outfitting = self.outfitting.as_ref()?;

        if timestamp >= &outfitting.timestamp {
            Some(outfitting)
        } else {
            None
        }
    }

    /// Returns the valid available shipyard based on the given timestamp.
    pub fn valid_shipyard(&self, timestamp: &DateTime<Utc>) -> Option<&Shipyard> {
        let shipyard = self.shipyard.as_ref()?;

        if timestamp >= &shipyard.timestamp {
            Some(shipyard)
        } else {
            None
        }
    }

    /// Returns the valid available market based on the given timestamp.
    pub fn valid_market(&self, timestamp: &DateTime<Utc>) -> Option<&Market> {
        let market = self.market.as_ref()?;

        if timestamp >= &market.timestamp {
            Some(market)
        } else {
            None
        }
    }

    /// Returns the valid available market based on the given timestamp.
    pub fn valid_backpack(&self, timestamp: &DateTime<Utc>) -> Option<&Backpack> {
        let backpack = self.backpack.as_ref()?;

        if timestamp >= &backpack.timestamp {
            Some(backpack)
        } else {
            None
        }
    }

    /// Returns the valid available ship locker based on the given timestamp.
    pub fn valid_ship_locker(&self, timestamp: &DateTime<Utc>) -> Option<&ShipLocker> {
        let ship_locker = self.ship_locker.as_ref()?;

        if timestamp >= &ship_locker.timestamp {
            Some(ship_locker)
        } else {
            None
        }
    }

    /// Returns the current available status about the player's ship.
    pub fn current_ship_status(&self) -> Option<&ShipStatus> {
        self.status
            .as_ref()?
            .contents
            .as_ref()?
            .kind
            .ship_status()
    }

    /// Returns the valid ship status based on the given timestamp.
    pub fn valid_ship_status(&self, timestamp: &DateTime<Utc>) -> Option<&ShipStatus> {
        self.valid_status(timestamp)
            .as_ref()?
            .contents
            .as_ref()?
            .kind
            .ship_status()
    }

    /// Returns the current status information about the planet the player is currently close to.
    pub fn current_planet_status(&self) -> Option<&PlanetStatus> {
        self.status
            .as_ref()?
            .contents
            .as_ref()?
            .planet_status
            .as_ref()
    }

    /// Returns the valid planet status based on the given timestamp.
    pub fn valid_planet_status(&self, timestamp: &DateTime<Utc>) -> Option<&PlanetStatus> {
        self.valid_status(timestamp)
            .as_ref()?
            .contents
            .as_ref()?
            .planet_status
            .as_ref()
    }
}


