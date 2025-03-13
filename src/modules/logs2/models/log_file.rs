use std::borrow::Cow;
use std::path::{Path, PathBuf};
use crate::modules::logs2::error::LogError;
use crate::modules::logs2::models::log_iter::LogIter;

#[cfg(feature = "asynchronous")]
use crate::modules::logs2::models::async_iter::AsyncIter;

pub struct LogFile<'a> {
    path: Cow<'a, Path>
}

impl<'a> LogFile<'a> {
    pub fn new<P: AsRef<Path>>(path: &'a P) -> LogFile<'a> {
        LogFile {
            path: Cow::Borrowed(path.as_ref())
        }
    }

    pub fn owned(path: PathBuf) -> LogFile<'static> {
        LogFile {
            path: Cow::Owned(path)
        }
    }

    pub fn iter(&self) -> Result<LogIter<std::io::BufReader<std::fs::File>>, LogError> {
        let file = std::fs::File::open(&self.path)?;
        let reader = std::io::BufReader::new(file);

        Ok(LogIter::from(reader))
    }

    #[cfg(feature = "asynchronous")]
    pub async fn async_iter(&self) -> Result<AsyncIter<tokio_util::compat::Compat<tokio::io::BufReader<tokio::fs::File>>>, LogError> {
        let file = tokio::fs::File::open(&self.path)
            .await?;

        let reader = tokio::io::BufReader::new(file);

        Ok(AsyncIter::from(reader))
    }
}