mod error;
mod models;

pub use error::LogIOError;
pub use models::log_iter::LogIter;
pub use models::log_path::LogPath;

#[cfg(feature = "asynchronous")]
pub use models::async_iter::AsyncIter;