//! Async variants of common use-cases and strategies.

mod async_log_file;
mod async_different_file;
mod async_newest_file;
mod async_changed_json_file;

pub use async_log_file::AsyncLogFile;
