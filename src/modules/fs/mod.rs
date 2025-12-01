mod functions;
mod models;
mod error;

pub use functions::auto_detect_journal_path::auto_detect_journal_path;

pub use models::log_dir::LogDir;
pub use models::log_dir_watcher::LogDirWatcher;
pub use models::log_file_watcher::LogFileWatcher;