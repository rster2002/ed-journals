use std::path::Path;

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;

use crate::journal::models::journal_event::JournalEvent;
use crate::journal::shared::journal_buffer::LiveJournalBuffer;
use crate::journal::LiveJournalBufferError;
use crate::logs::asynchronous::{LogDirReader, LogDirReaderError};
use crate::modules::journal::models::journal_event_kind::JournalEventKind;
use crate::modules::shared::asynchronous::async_blocker::AsyncBlocker;

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
#[deprecated]
pub struct LiveJournalDirReader {
    blocker: AsyncBlocker,
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
