mod functions;
mod models;
mod error;
mod traits;

pub use functions::auto_detect_journal_path::auto_detect_journal_path;

pub use error::LogFSError;
pub use models::log_dir::LogDir;
pub use models::dir_watcher::DirWatcher;
pub use models::file_watcher::FileWatcher;
pub use models::sync_blocker::SyncBlocker;

#[cfg(feature = "asynchronous")]
pub use models::async_blocker::AsyncBlocker;

type BlockResult = Result<(), LogFSError>;