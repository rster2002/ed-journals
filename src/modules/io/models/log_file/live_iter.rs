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
        let mut watcher = notify::recommended_watcher(move |_| {
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

        self.blocker.block();

        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::io::LiveIter;
    use crate::modules::shared::blocking::sync_blocker::SyncBlocker;
    use crate::tests::test_file;
    use std::fs;
    
    
    use std::thread::spawn;
    

    #[test]
    #[ignore]
    fn live_watcher_blocks_correctly() {
        let test_file = test_file();
        fs::write(test_file.path(), "").unwrap();

        let blocker = SyncBlocker::new();

        let local_blocker = blocker.clone();
        let local_path = test_file.path();
        let handle1 = spawn(move || {
            println!("Nope!");

            let mut live_reader = LiveIter::open(local_path).unwrap();

            local_blocker.unblock();
            assert!(dbg!(live_reader.next()).is_some());

            local_blocker.unblock();
            assert!(dbg!(live_reader.next()).is_some());
        });

        let local_blocker = blocker.clone();
        let handle2 = spawn(move || {
            println!("Haha first");
            local_blocker.block();

            fs::write(
                test_file.path(),
                r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#,
            )
                .unwrap();

            local_blocker.block();

            fs::write(test_file.path(), r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}
{"timestamp":"2022-10-22T15:12:05Z","event":"Commander","FID":"F123456789","Name":"TEST"}"#)
                .unwrap();

            println!("Haha second");
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }
}
