use std::sync::mpsc::Sender;
use crate::fs::{BlockResult, LogFSError};
use crate::fs::traits::unblocker::Unblocker;

/// Unblocks the associated blocker by sending a message to the associated channel.
#[derive(Debug, Clone)]
pub struct SyncUnblocker {
    inner: Sender<BlockResult>
}

impl SyncUnblocker {
    /// Creates a new unblocker using the provided sender.
    pub fn new(sender: Sender<BlockResult>) -> Self {
        Self {
            inner: sender
        }
    }
}

impl Unblocker for SyncUnblocker {
    fn unblock(&mut self, result: BlockResult) -> BlockResult {
        self.inner
            .send(result)
            .map_err(|_| LogFSError::FailedToUnblock)?;

        Ok(())
    }
}