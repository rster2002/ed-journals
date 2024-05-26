use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use notify::{Event, EventKind};
use notify::event::{CreateKind, DataChange, ModifyKind};
use thiserror::Error;
use crate::backpack::blocking::{read_backpack_file, ReadBackpackFileError};
use crate::cargo::blocking::{read_cargo_file, ReadCargoFileError};
use crate::journal::JournalEventKind;
use crate::journal::models::journal_event::JournalEvent;
use crate::market::blocking::{read_market_file, ReadMarketFileError};
use crate::modules_info::blocking::{read_modules_info_file, ReadModulesInfoFileError};
use crate::nav_route::blocking::{read_nav_route_file, ReadNavRouteFileError};
use crate::outfitting::blocking::{read_outfitting_file, ReadOutfittingFileError};
use crate::ship_locker::blocking::{read_ship_locker_file, ReadShipLockerFileError};
use crate::shipyard::blocking::{read_shipyard_file, ReadShipyardFileError};
use crate::status::blocking::{read_status_file, ReadStatusFileError};

/// This struct is largely to split off a large chunk of code that was shared between the blocking
/// and async variants of the live readers.
#[derive(Debug, Clone, Default)]
pub struct LiveJournalBuffer {
    dir_path: PathBuf,
    pending_events: Arc<Mutex<VecDeque<Result<JournalEvent, LiveJournalBufferError>>>>,
}

#[derive(Debug, Error)]
pub enum LiveJournalBufferError {
    #[error(transparent)]
    ReadStatusFileError(#[from] ReadStatusFileError),

    #[error(transparent)]
    ReadOutfittingFileError(#[from] ReadOutfittingFileError),

    #[error(transparent)]
    ReadShipyardFileError(#[from] ReadShipyardFileError),

    #[error(transparent)]
    ReadMarketFileError(#[from] ReadMarketFileError),

    #[error(transparent)]
    ReadNavRouteFileError(#[from] ReadNavRouteFileError),

    #[error(transparent)]
    ReadModulesInfoFileError(#[from] ReadModulesInfoFileError),

    #[error(transparent)]
    ReadBackpackFileError(#[from] ReadBackpackFileError),

    #[error(transparent)]
    ReadCargoFileError(#[from] ReadCargoFileError),

    #[error(transparent)]
    ReadShipLockerFileError(#[from] ReadShipLockerFileError),
}

impl LiveJournalBuffer {
    pub fn new(dir_path: PathBuf) -> Self {
        LiveJournalBuffer {
            dir_path,
            pending_events: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn handle_notify_event(&self, event: Event) {
        #[cfg(target_family = "unix")]
        match event.kind {
            EventKind::Create(CreateKind::File)
            | EventKind::Modify(ModifyKind::Data(DataChange::Content)) => true,
            _ => return,
        };

        #[cfg(target_family = "windows")]
        match event.kind {
            EventKind::Create(CreateKind::Any)
            | EventKind::Modify(ModifyKind::Any) => true,
            _ => return,
        };

        for path in event.paths {
            if path.ends_with("Status.json") {
                self.pending_events
                    .lock()
                    .expect("Failed to get lock")
                    .push_back(match read_status_file(self.dir_path.join("Status.json")) {
                        Ok(status) => Ok(JournalEvent {
                            is_live: true,
                            kind: JournalEventKind::StatusEvent(status),
                        }),
                        Err(error) => Err(error.into()),
                    });
            }

            if path.ends_with("Outfitting.json") {
                self.pending_events
                    .lock()
                    .expect("Failed to get lock")
                    .push_back(
                        match read_outfitting_file(self.dir_path.join("Outfitting.json")) {
                            Ok(status) => Ok(JournalEvent {
                                is_live: true,
                                kind: JournalEventKind::OutfittingEvent(status),
                            }),
                            Err(error) => Err(error.into()),
                        },
                    );
            }

            if path.ends_with("Shipyard.json") {
                self.pending_events
                    .lock()
                    .expect("Failed to get lock")
                    .push_back(match read_shipyard_file(self.dir_path.join("Shipyard.json")) {
                        Ok(shipyard) => Ok(JournalEvent {
                            is_live: true,
                            kind: JournalEventKind::ShipyardEvent(shipyard),
                        }),
                        Err(error) => Err(error.into()),
                    });
            }

            if path.ends_with("Market.json") {
                self.pending_events
                    .lock()
                    .expect("Failed to get lock")
                    .push_back(match read_market_file(self.dir_path.join("Market.json")) {
                        Ok(market) => Ok(JournalEvent {
                            is_live: true,
                            kind: JournalEventKind::MarketEvent(market),
                        }),
                        Err(error) => Err(error.into()),
                    });
            }

            if path.ends_with("NavRoute.json") {
                self.pending_events
                    .lock()
                    .expect("Failed to get lock")
                    .push_back(match read_nav_route_file(self.dir_path.join("NavRoute.json")) {
                        Ok(nav_route) => Ok(JournalEvent {
                            is_live: true,
                            kind: JournalEventKind::NavRoute(nav_route),
                        }),
                        Err(error) => Err(error.into()),
                    });
            }

            if path.ends_with("ModulesInfo.json") {
                self.pending_events
                    .lock()
                    .expect("Failed to get lock")
                    .push_back(
                        match read_modules_info_file(self.dir_path.join("ModulesInfo.json")) {
                            Ok(modules_info) => Ok(JournalEvent {
                                is_live: true,
                                kind: JournalEventKind::ModulesInfo(modules_info),
                            }),
                            Err(error) => Err(error.into()),
                        },
                    );
            }

            if path.ends_with("Backpack.json") {
                self.pending_events
                    .lock()
                    .expect("Failed to get lock")
                    .push_back(match read_backpack_file(self.dir_path.join("Backpack.json")) {
                        Ok(backpack) => Ok(JournalEvent {
                            is_live: true,
                            kind: JournalEventKind::Backpack(backpack),
                        }),
                        Err(error) => Err(error.into()),
                    });
            }

            if path.ends_with("Cargo.json") {
                self.pending_events
                    .lock()
                    .expect("Failed to get lock")
                    .push_back(match read_cargo_file(self.dir_path.join("Cargo.json")) {
                        Ok(cargo) => Ok(JournalEvent {
                            is_live: true,
                            kind: JournalEventKind::Cargo(cargo),
                        }),
                        Err(error) => Err(error.into()),
                    });
            }

            if path.ends_with("ShipLocker.json") {
                self.pending_events
                    .lock()
                    .expect("Failed to get lock")
                    .push_back(
                        match read_ship_locker_file(self.dir_path.join("ShipLocker.json")) {
                            Ok(ship_locker) => Ok(JournalEvent {
                                is_live: true,
                                kind: JournalEventKind::ShipLocker(ship_locker),
                            }),
                            Err(error) => Err(error.into()),
                        },
                    );
            }
        }
    }
}

impl Iterator for LiveJournalBuffer {
    type Item = Result<JournalEvent, LiveJournalBufferError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pending_events
            .lock()
            .expect("Failed to get lock")
            .pop_front()
    }
}
