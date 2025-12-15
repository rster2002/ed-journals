use notify::event::{CreateKind, ModifyKind};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use crate::fs::error::LogFSError;
use crate::fs::traits::blocker::Blocker;
use crate::fs::traits::unblocker::Unblocker;

/// Watches a file for changes and unblocks the associated blocker when a change occurs.
pub struct LogFileWatcher {
    _watcher: RecommendedWatcher,
}

impl LogFileWatcher {
    pub fn new<P: AsRef<Path>>(path: P, blocker: &impl Blocker) -> Result<LogFileWatcher, LogFSError> {
        let mut unblocker = blocker.unblocker();

        let mut watcher = notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
            let event: notify::Event = match event {
                Ok(event) => event,
                Err(error) => {
                    let _ = unblocker.unblock(Err(LogFSError::NotifyError(error)));
                    return;
                },
            };

            #[cfg(target_family = "unix")]
            match event.kind {
                EventKind::Create(CreateKind::File)
                | EventKind::Modify(ModifyKind::Data(_)) => true,
                _ => return,
            };

            #[cfg(target_family = "windows")]
            match event.kind {
                EventKind::Create(CreateKind::Any) | EventKind::Modify(ModifyKind::Any) => true,
                _ => return,
            };

            let _ = unblocker.unblock(Ok(()));
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LogFileWatcher {
            _watcher: watcher,
        })
    }
}