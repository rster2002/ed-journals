mod error;
mod functions;
mod models;

pub use error::LogError;

pub use models::log_path::LogPath;

pub use models::log_dir::dir_iter::DirIter;
pub use models::log_dir::live_dir_iter::LiveDirIter;
pub use models::log_dir::LogDir;

// #[cfg(feature = "asynchronous")]
// #[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
// pub use models::log_dir::async_live_dir_iter::AsyncLiveDirIter;

pub use models::log_file::live_iter::LiveIter;
pub use models::log_file::log_iter::LogIter;
pub use models::log_file::LogFile;

// #[cfg(feature = "asynchronous")]
// #[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
// pub use models::log_file::async_iter::AsyncIter;
//
// #[cfg(feature = "asynchronous")]
// #[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
// pub use models::log_file::live_async_iter::LiveAsyncIter;

pub use functions::auto_detect_journal_path::auto_detect_journal_path;

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::io::LogDir;

    #[test]
    fn scratch() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");

        let journal_dir = LogDir::new(&dir_path).iter().unwrap();

        let mut i = 0;
        for entry in journal_dir {
            let log_iter = entry.iter().unwrap();
            for log in log_iter {
                i += 1;
            }
        }

        dbg!(i);
    }
}
