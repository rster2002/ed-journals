use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::modules::shared::blocking::sync_blocker::SyncBlocker;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use super::{LiveLogDirHandle, LiveLogDirReaderError, RawLogDirReader};

/// Watches the whole journal dir and reads all files. Once all historic files have been read it
/// will block the current thread until the newest log file is changed at which it will read the
/// active log file and return the entry.
///
/// ```no_run
/// use std::path::PathBuf;
/// use ed_journals::logs::blocking::LiveLogDirReader;
///
/// let path = PathBuf::from("somePath");
///
/// let live_dir_reader = LiveLogDirReader::open(path)
///     .unwrap();
///
/// // At first this will read all existing lines from the journal logs, after which it will block
/// // the current thread until it detects new entries in the latest log file.
/// for entry in live_dir_reader {
///     // Do something with the entry
/// }
/// ```
#[derive(Debug)]
pub struct RawLiveLogDirReader {
    blocker: SyncBlocker,
    log_dir_reader: RawLogDirReader,
    _watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
}

impl RawLiveLogDirReader {
    pub fn open<P: AsRef<Path>>(dir_path: P) -> Result<RawLiveLogDirReader, LiveLogDirReaderError> {
        let log_dir_reader = RawLogDirReader::open(&dir_path);

        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock();
        })?;

        watcher.watch(dir_path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(Self {
            blocker,
            log_dir_reader,
            active: Arc::new(AtomicBool::new(true)),
            _watcher: watcher,
        })
    }

    pub fn handle(&self) -> LiveLogDirHandle {
        LiveLogDirHandle::new(self.active.clone(), self.blocker.clone())
    }
}

impl Iterator for RawLiveLogDirReader {
    type Item = Result<serde_json::Value, LiveLogDirReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.active.load(Ordering::Relaxed) || self.log_dir_reader.is_failing() {
                return None;
            }

            let Some(result) = self.log_dir_reader.next() else {
                self.blocker.block();
                continue;
            };

            return Some(result.map_err(|e| e.into()));
        }
    }
}
