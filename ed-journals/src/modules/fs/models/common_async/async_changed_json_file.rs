use crate::fs::common::JsonFile;
use crate::fs::{LogFSError, Unblocker};
use serde::de::DeserializeOwned;
use std::path::Path;
use std::sync::Arc;
use twox_hash::XxHash64;

/// Async variant of [ChangedJsonFile](crate::fs::common::ChangedJsonFile).
pub struct AsyncChangedJsonFile<R>
where
    R: DeserializeOwned,
{
    inner: JsonFile<R>,
    last_hash: Option<u64>,
}

impl<R> AsyncChangedJsonFile<R>
where
    R: DeserializeOwned + PartialEq,
{
    pub fn new<P: AsRef<Path>>(
        path: P,
        unblocker: impl Into<Arc<dyn Unblocker>>,
    ) -> Result<AsyncChangedJsonFile<R>, LogFSError> {
        Ok(AsyncChangedJsonFile {
            inner: JsonFile::new(path, unblocker)?,
            last_hash: None,
        })
    }

    /// Returns the current contents of the file as a deserialized object, or [None] if the contents
    /// haven't changed since the last read or if the file is empty (which happens when the game
    /// clears the file before it starts to write.)
    pub async fn content(&mut self) -> Result<Option<R>, LogFSError> {
        let bytes = self.inner.byte_content_async().await?;
        if bytes.is_empty() {
            return Ok(None);
        }

        let hash = XxHash64::oneshot(0, &bytes);

        if self.last_hash.is_some_and(|v| v == hash) {
            return Ok(None);
        }

        self.last_hash = Some(hash);
        Ok(Some(serde_json::from_slice(&bytes)?))
    }
}
