//! Collection of iterators for iterating over log entries. These come in both [sync](LogIter) and
//! [async](AsyncIter) variants and also have raw variants if you want to handle the JSON lines
//! yourself. The [LogPath] model is used to order the path names of logs and ensure they follow
//! the expected format.

mod error;
mod models;

pub use error::LogIOError;
pub use models::log_iter::LogIter;
pub use models::log_path::LogPath;

#[cfg(feature = "asynchronous")]
pub use models::async_iter::AsyncIter;
