use std::path::Path;
use std::sync::Arc;
use notify::RecommendedWatcher;
use crate::logs::LogEvent;
use crate::modules::logs2::{DirIter, LogError};
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

pub struct LiveDirIter {
    dir_iter: DirIter,
    blocker: SyncBlocker,
    _watcher: RecommendedWatcher,
}

impl LiveDirIter {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<LiveDirIter, LogError> {
        let dir_iter = DirIter::new(path)?;

        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        // This is stopped when it is dropped
        let mut watcher = notify::recommended_watcher(move |event| {
            let Ok(event) = event else {
                return;
            };

            local_blocker.unblock();
        })?;

        Ok(LiveDirIter {
            dir_iter,
            blocker,
            _watcher: watcher,
        })
    }
}

impl Iterator for LiveDirIter {
    type Item = Result<Arc<LogEvent>, LogError>;

    fn next(&mut self) -> Option<Self::Item> {
        
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::thread::spawn;
    use crate::modules::logs2::LiveIter;
    use crate::modules::logs2::models::log_dir::live_dir_iter::LiveDirIter;
    use crate::modules::shared::blocking::sync_blocker::SyncBlocker;
    use crate::tests::test_dir;

    #[test]
    fn live_watcher_blocks_correctly() {
        let dir = test_dir();
        let first_file = dir.file(0);

        fs::write(
            &first_file,
            r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#,
        )
            .unwrap();

        let blocker = SyncBlocker::new();

        let local_blocker = blocker.clone();
        let local_path = dir.path();

        let handle1 = spawn(move || {
            println!("Nope!");

            let mut live_dir = LiveDirIter::new(local_path).unwrap();

            assert!(dbg!(live_dir.next()).is_some());

            // let mut live_reader = LiveIter::open(local_path).unwrap();
            //
            // local_blocker.unblock();
            // assert!(dbg!(live_reader.next()).is_some());
            //
            // local_blocker.unblock();
            // assert!(dbg!(live_reader.next()).is_some());
        });
    }
}