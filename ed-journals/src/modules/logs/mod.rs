mod log_file;
mod log_dir;

/// Contains models for all the different events that are written in the log files.
pub mod content;

/// Contains readers for when working in a synchronous environment like a manually spawned thread.
#[cfg(feature = "blocking")]
pub mod blocking;

/// Contains readers for when working in an asynchronous environment like Tokio.
#[cfg(feature = "async")]
pub mod asynchronous;

pub use log_file::LogFile;
pub use log_file::LogFileError;
pub use log_dir::LogDir;
pub use log_dir::LogDirError;
