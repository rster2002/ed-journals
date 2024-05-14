use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::{DataChange, ModifyKind};
use thiserror::Error;
use crate::journal::journal_event::JournalEvent;
use crate::logs::blocking::{LogDirReader, LogDirReaderError};
use crate::modules::outfitting::blocking::{read_outfitting_file, ReadOutfittingFileError};
use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcherError;
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;
use crate::status::blocking::{read_status_file, ReadStatusFileError};

#[derive(Debug)]
pub struct LiveJournalDirReader {
    blocker: SyncBlocker,
    watcher: RecommendedWatcher,
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
    NotifyError(#[from] notify::Error),
}

impl LiveJournalDirReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, JournalDirWatcherError> {
        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        let dir_path = path.as_ref().to_path_buf();

        let pending_events = Arc::new(Mutex::new(VecDeque::new()));
        let local_pending_events = pending_events.clone();

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                let EventKind::Modify(ModifyKind::Data(DataChange::Content)) = event.kind else {
                    return;
                };

                for path in event.paths {
                    if path.ends_with("Status.json") {
                        local_pending_events.lock()
                            .expect("Failed to get lock")
                            .push_back(match read_status_file(dir_path.join("Status.json")) {
                                Ok(status) => Ok(JournalEvent::StatusEvent(status)),
                                Err(error) => Err(error.into()),
                            });
                    }

                    if path.ends_with("Outfitting.json") {
                        local_pending_events.lock()
                            .expect("Failed to get lock")
                            .push_back(match read_outfitting_file(dir_path.join("Outfitting.json")) {
                                Ok(status) => Ok(JournalEvent::OutfittingEvent(status)),
                                Err(error) => Err(error.into()),
                            });
                    }
                }

                local_blocker.unblock();
            }
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        let log_dir_reader = LogDirReader::open(path);

        Ok(LiveJournalDirReader {
            blocker,
            watcher,
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
                })
            }

            let result = self.pending_events.lock()
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

