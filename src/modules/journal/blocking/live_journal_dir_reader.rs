use std::collections::VecDeque;
use std::path::Path;
use std::sync::{Arc, Mutex};

use notify::event::{CreateKind, DataChange, ModifyKind};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;

use crate::backpack::blocking::{read_backpack_file, ReadBackpackFileError};
use crate::journal::journal_event::JournalEvent;
use crate::logs::blocking::{LogDirReader, LogDirReaderError};
use crate::market::blocking::{read_market_file, ReadMarketFileError};
use crate::modules::cargo::blocking::{read_cargo_file, ReadCargoFileError};
use crate::modules::outfitting::blocking::{read_outfitting_file, ReadOutfittingFileError};
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;
use crate::modules_info::blocking::{read_modules_info_file, ReadModulesInfoFileError};
use crate::nav_route::blocking::{read_nav_route_file, ReadNavRouteFileError};
use crate::ship_locker::blocking::{read_ship_locker_file, ReadShipLockerFileError};
use crate::shipyard::blocking::{read_shipyard_file, ReadShipyardFileError};
use crate::status::blocking::{read_status_file, ReadStatusFileError};

/// Watches the entire journal directory for changes and emits then as events. The reader will
/// initially read all the history logs until it reaches the end of the latest log file, at
/// which it will block the current thread until there are any updates. At any point during
/// this if there are changes to any of the `.json` files in the log directory, these events
/// will be fired first before continuing reading the log files.
///
/// ```rust
/// # use std::env::current_dir;
/// use std::path::PathBuf;
/// use ed_journals::journal::blocking::LiveJournalDirReader;
///
/// let path = PathBuf::from("somePath");
/// # let path = current_dir()
/// #    .unwrap()
/// #    .join("test-files")
/// #    .join("journals");
/// let mut journal_reader = LiveJournalDirReader::open(&path).unwrap();
///
/// for entry in journal_reader {
///     // Do something with the entry
///     # break;
/// }
/// ```
#[derive(Debug)]
pub struct LiveJournalDirReader {
    blocker: SyncBlocker,
    _watcher: RecommendedWatcher,
    log_dir_reader: LogDirReader,
    pending_events: Arc<Mutex<VecDeque<Result<JournalEvent, JournalDirWatcherError>>>>,
}

#[derive(Debug, Error)]
pub enum JournalDirWatcherError {
    #[error(transparent)]
    LogDirReaderError(#[from] LogDirReaderError),

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

    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl LiveJournalDirReader {
    /// Attempts to open the given path and tries to start watching the contents. Note that this
    /// does not check if the path actually points to a correct journal directory.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, JournalDirWatcherError> {
        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        let dir_path = path.as_ref().to_path_buf();

        let pending_events = Arc::new(Mutex::new(VecDeque::new()));
        let local_pending_events = pending_events.clone();

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                match event.kind {
                    EventKind::Create(CreateKind::File)
                    | EventKind::Modify(ModifyKind::Data(DataChange::Content)) => true,
                    _ => return,
                };

                for path in event.paths {
                    if path.ends_with("Status.json") {
                        local_pending_events
                            .lock()
                            .expect("Failed to get lock")
                            .push_back(match read_status_file(dir_path.join("Status.json")) {
                                Ok(status) => Ok(JournalEvent::StatusEvent(status)),
                                Err(error) => Err(error.into()),
                            });
                    }

                    if path.ends_with("Outfitting.json") {
                        local_pending_events
                            .lock()
                            .expect("Failed to get lock")
                            .push_back(
                                match read_outfitting_file(dir_path.join("Outfitting.json")) {
                                    Ok(status) => Ok(JournalEvent::OutfittingEvent(status)),
                                    Err(error) => Err(error.into()),
                                },
                            );
                    }

                    if path.ends_with("Shipyard.json") {
                        local_pending_events
                            .lock()
                            .expect("Failed to get lock")
                            .push_back(match read_shipyard_file(dir_path.join("Shipyard.json")) {
                                Ok(shipyard) => Ok(JournalEvent::ShipyardEvent(shipyard)),
                                Err(error) => Err(error.into()),
                            });
                    }

                    if path.ends_with("Market.json") {
                        local_pending_events
                            .lock()
                            .expect("Failed to get lock")
                            .push_back(match read_market_file(dir_path.join("Market.json")) {
                                Ok(market) => Ok(JournalEvent::MarketEvent(market)),
                                Err(error) => Err(error.into()),
                            });
                    }

                    if path.ends_with("NavRoute.json") {
                        local_pending_events
                            .lock()
                            .expect("Failed to get lock")
                            .push_back(match read_nav_route_file(dir_path.join("NavRoute.json")) {
                                Ok(nav_route) => Ok(JournalEvent::NavRoute(nav_route)),
                                Err(error) => Err(error.into()),
                            });
                    }

                    if path.ends_with("ModulesInfo.json") {
                        local_pending_events
                            .lock()
                            .expect("Failed to get lock")
                            .push_back(
                                match read_modules_info_file(dir_path.join("ModulesInfo.json")) {
                                    Ok(modules_info) => Ok(JournalEvent::ModulesInfo(modules_info)),
                                    Err(error) => Err(error.into()),
                                },
                            );
                    }

                    if path.ends_with("Backpack.json") {
                        local_pending_events
                            .lock()
                            .expect("Failed to get lock")
                            .push_back(match read_backpack_file(dir_path.join("Backpack.json")) {
                                Ok(backpack) => Ok(JournalEvent::Backpack(backpack)),
                                Err(error) => Err(error.into()),
                            });
                    }

                    if path.ends_with("Cargo.json") {
                        local_pending_events
                            .lock()
                            .expect("Failed to get lock")
                            .push_back(match read_cargo_file(dir_path.join("Cargo.json")) {
                                Ok(cargo) => Ok(JournalEvent::Cargo(cargo)),
                                Err(error) => Err(error.into()),
                            });
                    }

                    if path.ends_with("ShipLocker.json") {
                        local_pending_events
                            .lock()
                            .expect("Failed to get lock")
                            .push_back(
                                match read_ship_locker_file(dir_path.join("ShipLocker.json")) {
                                    Ok(ship_locker) => Ok(JournalEvent::ShipLocker(ship_locker)),
                                    Err(error) => Err(error.into()),
                                },
                            );
                    }
                }

                local_blocker.unblock();
            }
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        let log_dir_reader = LogDirReader::open(path);

        Ok(LiveJournalDirReader {
            blocker,
            _watcher: watcher,
            log_dir_reader,
            pending_events,
        })
    }
}

impl Iterator for LiveJournalDirReader {
    type Item = Result<JournalEvent, JournalDirWatcherError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(log_event) = self.log_dir_reader.next() {
                return Some(match log_event {
                    Ok(event) => Ok(JournalEvent::LogEvent(event)),
                    Err(error) => Err(error.into()),
                });
            }

            let result = self
                .pending_events
                .lock()
                .expect("Failed to get lock")
                .pop_front();

            if result.is_none() {
                self.blocker.block();
                continue;
            }

            return result;
        }
    }
}
