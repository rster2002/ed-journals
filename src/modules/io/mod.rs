mod error;
mod models;

pub use error::LogError;
pub use models::log_iter::LogIter;

#[cfg(feature = "asynchronous")]
pub use models::async_iter::AsyncIter;
