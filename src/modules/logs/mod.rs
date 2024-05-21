pub use log_dir::LogDir;
pub use log_dir::LogDirError;
pub use log_file::LogFile;
pub use log_file::LogFileError;

mod log_dir;
mod log_file;

/// Contains models for all the different events that are written in the log files.
pub mod content;

/// Contains readers for when working in a synchronous environment like a manually spawned thread.
pub mod blocking;

/// Contains readers for when working in an asynchronous environment like Tokio.
#[cfg(feature = "asynchronous")]
pub mod asynchronous;
