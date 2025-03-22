use crate::modules::io::{DirIter, LogError, LogFile, LogPath};
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;
use notify::event::CreateKind;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::VecDeque;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct LiveDirIter {
    dir_iter: DirIter,
    blocker: SyncBlocker,
    last: Option<LogPath>,
    added: Arc<Mutex<VecDeque<Result<LogFile, LogError>>>>,
    _watcher: RecommendedWatcher,
}

impl LiveDirIter {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<LiveDirIter, LogError> {
        let dir_iter = DirIter::new(path.as_ref())?;

        let blocker = SyncBlocker::new();
        let added = Arc::new(Mutex::new(VecDeque::new()));

        let local_blocker = blocker.clone();
        let local_added = added.clone();

        // This is stopped when it is dropped
        let mut watcher = notify::recommended_watcher(move |event: notify::Result<Event>| {
            let Ok(event) = event else {
                return;
            };

            if !matches!(event.kind, EventKind::Create(CreateKind::File)) {
                return;
            }

            let mut lock = local_added.lock().expect("lock should have been acquired");

            for path in event.paths {
                let path = match LogPath::try_from(path.as_path()) {
                    Ok(path) => path,
                    Err(LogError::IncorrectFileName) => continue,
                    Err(error) => {
                        lock.push_back(Err(error));
                        continue;
                    }
                };

                let log_file = LogFile::from(path);

                lock.push_back(Ok(log_file));
            }

            local_blocker.unblock();
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LiveDirIter {
            dir_iter,
            blocker,
            added,
            last: None,
            _watcher: watcher,
        })
    }

    pub fn is_last(&self, log_file: &LogFile) -> bool {
        let Some(last) = &self.last else {
            return false;
        };

        log_file == last
    }
}

impl Iterator for LiveDirIter {
    type Item = Result<LogFile, LogError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut entry) = self.dir_iter.next() {
            self.last = Some(entry.log_path().clone());
            entry.set_blocker(Arc::new(self.blocker.clone()));

            return Some(Ok(entry));
        }

        loop {
            let added_value = self
                .added
                .lock()
                .expect("lock should have been acquired")
                .pop_front();

            if added_value.is_some() {
                return added_value;
            }

            self.blocker.block();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::io::models::log_dir::live_dir_iter::LiveDirIter;
    use crate::modules::shared::blocking::sync_blocker::SyncBlocker;
    use crate::tests::test_dir;
    use std::fs;
    use std::thread::spawn;

    #[test]
    #[ignore]
    fn live_watcher_blocks_correctly() {
        let dir = test_dir();
        let first_file = dir.path().join("Journal.2023-02-21T084116.01.log");
        let second_file = dir.path().join("Journal.2023-02-21T084116.02.log");

        fs::write(
            &first_file,
            r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#,
        )
            .unwrap();

        let blocker = SyncBlocker::new();

        let local_blocker = blocker.clone();
        let local_path = dir.path();

        let handle1 = spawn(move || {
            let mut live_dir = LiveDirIter::new(local_path).unwrap();
            let mut file = live_dir.next().unwrap().unwrap().live_iter().unwrap();

            assert!(file.next().is_some());

            local_blocker.unblock();
            assert!(file.next().is_none());

            let mut next_file = live_dir.next().unwrap().unwrap().iter().unwrap();

            assert!(next_file.next().is_some());
            assert!(next_file.next().is_none());

            assert!(true);
        });

        let handle2 = spawn(move || {
            blocker.block();

            fs::write(
                &second_file,
                r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#,
            )
                .unwrap();
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }
}
