use std::path::Path;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::{CreateKind, RemoveKind};
use crate::fs::error::LogFSError;
use crate::fs::traits::blocker::Blocker;

/// Watches a directory for changes and unblocks the associated blocker when a change occurs. Takes
/// a path and any [Blocker]:
///
/// ```rust
/// use std::env::current_dir;
/// use ed_journals::fs::{auto_detect_journal_path, DirWatcher, SyncBlocker};
///
/// let path = current_dir()
///     .unwrap()
///     .join("test-files")
///     .join("journals");
///
/// let blocker = SyncBlocker::new();
/// let dir_watcher = DirWatcher::new(&path, &blocker).unwrap();
///
/// # return;
/// blocker.block().unwrap();
/// // Something changed
/// ```
pub struct DirWatcher {
    _watcher: RecommendedWatcher,
}

impl DirWatcher {
    pub fn new<P: AsRef<Path>>(path: P, blocker: &impl Blocker) -> Result<DirWatcher, LogFSError> {
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
                | EventKind::Remove(RemoveKind::Any) => true,
                _ => return,
            };

            #[cfg(target_family = "windows")]
            match event.kind {
                EventKind::Create(CreateKind::Any) | EventKind::Remove(RemoveKind::Any) => true,
                _ => return,
            };

            let _ = unblocker.unblock(Ok(()));
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(DirWatcher {
            _watcher: watcher,
        })
    }
}