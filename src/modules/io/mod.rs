mod error;
mod functions;
mod models;

pub use error::LogError;

pub use crate::modules::queue::models::dir_entry::DirEntry;

pub use models::log_dir::dir_iter::DirIter;
pub use models::log_dir::live_dir_iter::LiveDirIter;
pub use models::log_dir::LogDir;

#[cfg(feature = "asynchronous")]
#[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
pub use models::log_dir::async_live_dir_iter::AsyncLiveDirIter;

pub use models::log_file::live_iter::LiveIter;
pub use models::log_file::log_iter::LogIter;
pub use models::log_file::LogFile;

#[cfg(feature = "asynchronous")]
#[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
pub use models::log_file::async_iter::AsyncIter;

#[cfg(feature = "asynchronous")]
#[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
pub use models::log_file::live_async_iter::LiveAsyncIter;

pub use functions::auto_detect_journal_path::auto_detect_journal_path;
