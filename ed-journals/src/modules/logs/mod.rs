mod log_file;
mod log_dir;
pub mod content;

#[cfg(feature = "blocking")]
pub mod blocking;

#[cfg(feature = "async")]
pub mod r#async;

pub use log_file::LogFile;
pub use log_file::LogFileError;
pub use log_dir::LogDir;
pub use log_dir::LogDirError;
