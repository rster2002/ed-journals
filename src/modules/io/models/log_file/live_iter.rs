use crate::logs::LogEvent;
use crate::modules::io::error::LogError;
use crate::modules::io::LogIter;
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;
use notify::{RecommendedWatcher, Watcher};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub struct LiveIter {
    inner: LogIter<BufReader<File>>,
    blocker: Arc<SyncBlocker>,
    _watcher: RecommendedWatcher,
}

impl LiveIter {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<LiveIter, LogError> {
        LiveIter::with_blocker(path, Arc::new(SyncBlocker::new()))
    }

    pub(crate) fn with_blocker<P: AsRef<Path>>(
        path: P,
        blocker: Arc<SyncBlocker>,
    ) -> Result<LiveIter, LogError> {
        let file = File::open(&path)?;
        let buf_reader = BufReader::new(file);
        let log_iter = LogIter::from(buf_reader);

        let local_blocker = blocker.clone();

        // This is stopped when it is dropped
        let mut watcher = notify::recommended_watcher(move |event| {
            dbg!(&event);
            local_blocker.unblock();
        })?;

        watcher.watch(path.as_ref(), notify::RecursiveMode::NonRecursive)?;

        Ok(LiveIter {
            inner: log_iter,
            blocker,
            _watcher: watcher,
        })
    }

    pub fn blocker(&self) -> &SyncBlocker {
        &self.blocker
    }
}

impl Iterator for LiveIter {
    type Item = Result<LogEvent, LogError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.inner.next() {
            return Some(event);
        }

        loop {
            self.blocker.block();

            let entry = self.inner.next();
            
            if entry.is_some() {
                return entry;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::io::LiveIter;
    use std::time::Instant;
    use crate::modules::tests::simulate_log_file;

    #[test]
    fn live_watcher_blocks_correctly() {
        let path = simulate_log_file("live_watcher_blocks_correctly");
        let live_reader = LiveIter::open(&path).unwrap();

        let mut i = 0;
        let mut instant = Instant::now();
        for entry in live_reader {
            i += 1;
            assert!(entry.is_ok());

            // Simulation sleeps for 100 ms, so if ~100 ms have passed, we can be sure that the
            // blocking has worked.
            assert!(instant.elapsed().as_millis() > 90);

            instant = Instant::now();
            
            if i > 20 {
                return;
            }
        }

        // We should never get here.
        unreachable!();
    }
}
