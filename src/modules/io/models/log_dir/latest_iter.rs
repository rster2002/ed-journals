use std::path::Path;
use std::sync::Arc;
use notify::{Event, EventKind};
use notify::event::CreateKind;
use crate::io::{DirIter, LiveDirIter, LogError, LogFile, LogPath};
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

// Behavior example:
// - call .next returns the current latest log file
// - call .next again will block until a new log file is created that is later than the current one.
//   the current log file should be unblocked.

#[derive(Debug)]
pub struct LatestIter {
    current: Option<LogFile>,
}

impl LatestIter {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<LatestIter, LogError> {
        let mut dir_iter = DirIter::new(path.as_ref())?;
        let current = dir_iter.last();

        let blocker = SyncBlocker::new();

        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |event: notify::Result<Event>| {
            let Ok(event) = event else {
                return;
            };

            if !matches!(event.kind, EventKind::Create(CreateKind::File)) {
                return;
            }
        });

        Ok(LatestIter {
            current,
        })
    }
}

impl Iterator for LatestIter {
    type Item = Result<LogFile, LogError>;

    fn next(&mut self) -> Option<Self::Item> {


        todo!()
    }
}
