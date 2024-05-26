use std::collections::VecDeque;
use std::path::Path;
use std::sync::{Arc, Mutex};

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::{DataChange, ModifyKind};
use thiserror::Error;

use crate::backpack::blocking::{read_backpack_file, ReadBackpackFileError};
use crate::modules::journal::models::journal_event_kind::JournalEventKind;
use crate::journal::LiveJournalBufferError;
use crate::journal::models::journal_event::JournalEvent;
use crate::journal::shared::journal_buffer::LiveJournalBuffer;
use crate::logs::asynchronous::{LogDirReader, LogDirReaderError};
use crate::market::blocking::{read_market_file, ReadMarketFileError};
use crate::modules::cargo::blocking::{read_cargo_file, ReadCargoFileError};
use crate::modules::outfitting::blocking::{read_outfitting_file, ReadOutfittingFileError};
use crate::modules::shared::asynchronous::async_blocker::AsyncBlocker;
use crate::modules_info::blocking::{read_modules_info_file, ReadModulesInfoFileError};
use crate::nav_route::blocking::{read_nav_route_file, ReadNavRouteFileError};
use crate::ship_locker::blocking::{read_ship_locker_file, ReadShipLockerFileError};
use crate::shipyard::blocking::{read_shipyard_file, ReadShipyardFileError};
use crate::status::blocking::{read_status_file, ReadStatusFileError};

// TODO this is in need of some abstracting
/// Asynchronous variant of the blocking [LiveJournalDirReader](BlockingLiveJournalDirReader).
/// Largely works the same way, but instead of completely blocking the current thread, it instead
/// will resolve a future when new events are fired.
///
/// ```rust
/// # use std::env::current_dir;
/// use std::path::PathBuf;
/// use ed_journals::journal::asynchronous::LiveJournalDirReader;
///
/// # tokio_test::block_on(async {
/// let path = PathBuf::from("somePath");
/// # let path = current_dir()
/// #    .unwrap()
/// #    .join("test-files")
/// #    .join("journals");
/// let mut journal_reader = LiveJournalDirReader::open(&path).unwrap();
///
/// while let Some(entry) = journal_reader.next().await {
///     // Do something with the entry
///     # break;
/// }
/// # });
/// ```
#[derive(Debug)]
pub struct LiveJournalDirReader {
    blocker: AsyncBlocker,
    _watcher: RecommendedWatcher,
    log_dir_reader: LogDirReader,
    journal_buffer: LiveJournalBuffer,
}

#[derive(Debug, Error)]
pub enum JournalDirWatcherError {
    #[error(transparent)]
    LogDirReaderError(#[from] LogDirReaderError),

    #[error(transparent)]
    LiveJournalBufferError(#[from] LiveJournalBufferError),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl LiveJournalDirReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, JournalDirWatcherError> {
        let blocker = AsyncBlocker::new();
        let local_blocker = blocker.clone();

        let dir_path = path.as_ref().to_path_buf();

        let journal_buffer = LiveJournalBuffer::new(dir_path);
        let local_journal_buffer = journal_buffer.clone();

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                local_journal_buffer.handle_notify_event(event);
                local_blocker.unblock_blocking();
            }
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        let log_dir_reader = LogDirReader::open(path);

        Ok(LiveJournalDirReader {
            blocker,
            _watcher: watcher,
            log_dir_reader,
            journal_buffer,
        })
    }

    pub async fn next(&mut self) -> Option<Result<JournalEvent, JournalDirWatcherError>> {
        loop {
            if let Some(log_event) = self.log_dir_reader.next().await {
                return Some(match log_event {
                    Ok(event) => Ok(JournalEvent {
                        is_live: self.log_dir_reader.is_reading_latest(),
                        kind: JournalEventKind::LogEvent(event),
                    }),
                    Err(error) => Err(error.into()),
                });
            }

            match self.journal_buffer.next() {
                Some(Ok(entry)) => return Some(Ok(entry)),
                Some(Err(error)) => return Some(Err(error.into())),
                None => {
                    self.blocker.block().await;
                    continue;
                }
            }
        }
    }
}
