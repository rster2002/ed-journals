use crate::fs::error::LogFSError;
use crate::fs::traits::unblocker::Unblocker;
use notify::event::{CreateKind, ModifyKind};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::sync::Arc;

/// Watches a file for changes and unblocks the provided blocker when a change occurs.
///
/// ```rust
/// use std::env::current_dir;
/// use ed_journals::fs::{auto_detect_journal_path, FileWatcher, SyncBlocker};
///
/// let path = current_dir()
///     .unwrap()
///     .join("test-files")
///     .join("journals")
///     .join("Journal.2000-01-01T100000.01.log");
///
/// let blocker = SyncBlocker::new();
/// let file_watcher = FileWatcher::new(&path, &blocker).unwrap();
///
/// # return;
/// blocker.wait().unwrap();
/// // Something changed
/// ```
///
/// Keep in mind that this watcher needs to be kept in scope for as long as you want to receive
/// notifications.
pub struct FileWatcher {
    _watcher: RecommendedWatcher,
}

impl FileWatcher {
    /// Creates a new [FileWatcher] which will watch the provided path for changes.
    pub fn new<P: AsRef<Path>>(
        path: P,
        unblocker: impl Into<Arc<dyn Unblocker>>,
    ) -> Result<FileWatcher, LogFSError> {
        let unblocker = unblocker.into();

        let mut watcher =
            notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
                let event: notify::Event = match event {
                    Ok(event) => event,
                    Err(error) => {
                        let _ = unblocker.unblock(Err(LogFSError::NotifyError(error)));
                        return;
                    }
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

        Ok(FileWatcher { _watcher: watcher })
    }
}

impl Debug for FileWatcher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileWatcher")
    }
}
