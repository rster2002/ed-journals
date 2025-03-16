use std::collections::VecDeque;
use std::path::Path;
use std::sync::{Arc, Mutex, Weak};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::CreateKind;
use crate::logs::LogEvent;
use crate::modules::logs2::{DirIter, LogError, LogFile, LogPath};
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

pub struct LiveDirIter {
    dir_iter: DirIter,
    blocker: SyncBlocker,
    added: Arc<Mutex<VecDeque<Result<LogFile, LogError>>>>,
    _watcher: RecommendedWatcher,
}

impl LiveDirIter {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<LiveDirIter, LogError> {
        let dir_iter = DirIter::new(path.as_ref())?;

        let added = Arc::new(Mutex::new(VecDeque::new()));
        let blocker = SyncBlocker::new();

        let local_blocker = blocker.clone();
        let local_added = added.clone();

        // This is stopped when it is dropped
        let mut watcher = notify::recommended_watcher(move |event: notify::Result<Event>| {
            let Ok(event) = event else {
                return;
            };

            if !matches!(event, EventKind::Create(CreateKind::File)) {
                return;
            }

            for path in event.paths {
                let path = match LogPath::try_from(path.as_path()) {
                    Ok(path) => path,
                    Err(LogError::IncorrectFileName) => continue,
                    Err(error) => {
                        todo!()
                    }
                };
            }

            // dbg!(&event);
            // let Ok(event) = event else {
            //     return;
            // };

            local_blocker.unblock();
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LiveDirIter {
            dir_iter,
            blocker,
            added: VecDeque::new(),
            _watcher: watcher,
        })
    }
}

impl Iterator for LiveDirIter {
    type Item = Result<LogFile, LogError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut entry) = self.dir_iter.next() {
            entry.set_blocker(self.blocker.child());

            return Some(Ok(entry))
        }

        self.blocker.block();

        self.added.pop_front().map(Ok)
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
            println!("Nope!");

            let mut live_dir = LiveDirIter::new(local_path).unwrap();
            let mut file = live_dir.next()
                .unwrap()
                .unwrap()
                .iter()
                .unwrap();

            assert!(file.next().is_some());

            local_blocker.unblock();
            assert!(dbg!(live_dir.next()).is_some());

            assert!(true);

            // let mut live_reader = LiveIter::open(local_path).unwrap();
            //
            // local_blocker.unblock();
            // assert!(dbg!(live_reader.next()).is_some());
            //
            // local_blocker.unblock();
            // assert!(dbg!(live_reader.next()).is_some());
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
    }
}