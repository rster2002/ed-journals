pub mod dir_iter;
pub mod log_path;

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use crate::modules::logs2::LogError;
use crate::modules::logs2::models::log_dir::dir_iter::DirIter;

pub struct LogDir {
    path: PathBuf,
}

impl LogDir {
    pub fn new<P: AsRef<Path>>(path: P) -> LogDir {
        LogDir {
            path: path.as_ref().to_path_buf()
        }
    }

    pub fn iter(&self) -> Result<DirIter, LogError> {
        DirIter::new(&self.path)
    }
}