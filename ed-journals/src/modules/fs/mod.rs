//! This module contains utilities for working with the file system and handling changes in the log
//! directory. Using the different components, you can customize how you want events to be received.
//!
//! Most strategies involve blocking until there are new entries available. This is done using the
//! [SyncBlocker](crate::fs::SyncBlocker) or [AsyncBlocker](crate::fs::AsyncBlocker). These can be used to block execution until a watcher unblocks
//! them.
//!
//! Watchers are used to keep track of file system changes, either for the whole log directory using
//! the [DirWatcher](crate::fs::DirWatcher) or a single file using the [FileWatcher](crate::fs::FileWatcher). These require
//! a blocker to be passed when constructing them (it doesn't matter which blocker is passed,
//! watchers don't care if you're running sync or async code) and when something changes, they
//! signal to unblock.
//!
//! The actual interaction with the log directory is facilitated using the [LogDir] iterator. It
//! iterates over all the log files in the correct order and returns the
//! [LogPath](crate::io::LogPath) which can be opened however you want and read using the various
//! iterators available in the [io module](crate::io).
//!
//! The [common](common) module contains some default implementations for reading log files and JSON
//! files.

mod error;
mod functions;
mod models;
mod traits;

pub use functions::auto_detect_journal_path::auto_detect_journal_path;

pub use error::LogFSError;
pub use models::common;
pub use models::dir_watcher::DirWatcher;
pub use models::file_watcher::FileWatcher;
pub use models::log_dir::LogDir;
pub use models::sync_blocker::sync_unblocker::SyncUnblocker;
pub use models::sync_blocker::SyncBlocker;

pub use traits::blocker::Blocker;
pub use traits::unblocker::Unblocker;

#[cfg(feature = "asynchronous")]
pub use models::async_blocker::AsyncBlocker;

#[cfg(feature = "asynchronous")]
pub use models::async_blocker::async_unblocker::AsyncUnblocker;

#[cfg(feature = "asynchronous")]
pub use models::common_async;

type BlockResult = Result<(), LogFSError>;
