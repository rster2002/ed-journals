use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::de::DeserializeOwned;
use crate::fs::{FileWatcher, LogFSError, Unblocker};

pub struct JsonFile<R>
where R : DeserializeOwned,
{
    path: PathBuf,
    file_watcher: FileWatcher,
    _p: PhantomData<R>,
}

impl<R> JsonFile<R>
where R : DeserializeOwned,
{
    pub fn new<P: AsRef<Path>>(path: P, unblocker: impl Into<Arc<dyn Unblocker>>) -> Result<JsonFile<R>, LogFSError> {
        let path = path.as_ref();
        let file_watcher = FileWatcher::new(&path, unblocker)?;

        Ok(JsonFile {
            path: path.to_path_buf(),
            file_watcher,
            _p: PhantomData,
        })
    }

    pub fn byte_content(&self) -> Result<Vec<u8>, LogFSError> {
        Ok(std::fs::read(&self.path)?)
    }

    pub fn string_content(&self) -> Result<String, LogFSError> {
        Ok(std::fs::read_to_string(&self.path)?)
    }

    /// Returns the current contents of the file as a deserialized object, or [None] if the file
    /// is empty (which happens when the game clears the file before it starts to write.)
    pub fn content(&self) -> Result<Option<R>, LogFSError> {
        let contents = self.byte_content()?;

        if contents.is_empty() {
            return Ok(None);
        }

        Ok(Some(serde_json::from_slice(&contents)?))
    }

    #[cfg(feature = "asynchronous")]
    pub async fn byte_content_async(&self) -> Result<Vec<u8>, LogFSError> {
        Ok(async_fs::read(&self.path).await?)
    }

    #[cfg(feature = "asynchronous")]
    pub async fn string_content_async(&self) -> Result<String, LogFSError> {
        Ok(async_fs::read_to_string(&self.path).await?)
    }

    #[cfg(feature = "asynchronous")]
    pub async fn content_async(&self) -> Result<Option<R>, LogFSError> {
        let contents = self.byte_content_async().await?;

        if contents.is_empty() {
            return Ok(None);
        }

        Ok(Some(serde_json::from_slice(&contents)?))
    }
}