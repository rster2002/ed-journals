//! Types for common use-cases and strategies.

mod different_file;
mod log_file;
mod newest_file;
mod json_file;
mod changed_json_file;

/// Holds both a [FileWatcher](crate::fs::FileWatcher) and an [LogIter](crate::io::LogIter) over
/// a buf read file.
pub use log_file::LogFile;

/// Container which holds a [LogFile] and that changes when calling [DifferentFile::maybe_switch]
/// with a path other than the current one.
pub use different_file::DifferentFile;

/// Container which holds a [LogFile] and that changes when calling [NewestFile::maybe_next]
/// with a [LogPath](crate::io::LogPath) which is newer than the current one.
pub use newest_file::NewestFile;

pub use json_file::JsonFile;
pub use changed_json_file::ChangedJsonFile;
