use std::path::Path;

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;

use crate::journal::models::journal_event::JournalEvent;
use crate::journal::shared::journal_buffer::LiveJournalBuffer;
use crate::journal::LiveJournalBufferError;
use crate::logs::blocking::{LogDirReader, LogDirReaderError};
use crate::modules::journal::models::journal_event_kind::JournalEventKind;
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

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
    journal_buffer: LiveJournalBuffer,
}

#[derive(Debug, Error)]
pub enum JournalDirWatcherError {
    #[error(transparent)]
    LogDirReader(#[from] LogDirReaderError),

    #[error(transparent)]
    LiveJournalBuffer(#[from] LiveJournalBufferError),

    #[error(transparent)]
    Notify(#[from] notify::Error),
}

impl LiveJournalDirReader {
    /// Attempts to open the given path and tries to start watching the contents. Note that this
    /// does not check if the path actually points to a correct journal directory.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, JournalDirWatcherError> {
        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        let dir_path = path.as_ref().to_path_buf();

        let journal_buffer = LiveJournalBuffer::new(dir_path);
        let local_journal_buffer = journal_buffer.clone();

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                local_journal_buffer.handle_notify_event(event);
                local_blocker.unblock();
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
}

impl Iterator for LiveJournalDirReader {
    type Item = Result<JournalEvent, JournalDirWatcherError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(log_event) = self.log_dir_reader.next() {
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
                    self.blocker.block();
                    continue;
                }
            }
        }
    }
}
