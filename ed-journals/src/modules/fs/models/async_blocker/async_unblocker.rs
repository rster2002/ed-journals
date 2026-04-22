use crate::fs::traits::unblocker::Unblocker;
use crate::fs::{BlockResult, LogFSError};
use futures::channel::mpsc::Sender;
use std::sync::{Arc, Mutex};

/// Unblocks the associated [AsyncBlocker](crate::fs::AsyncBlocker) by sending a message to the
/// associated channel.
#[derive(Clone)]
pub struct AsyncUnblocker {
    sender: Arc<Mutex<Sender<BlockResult>>>,
}

impl AsyncUnblocker {
    pub fn new(sender: Sender<BlockResult>) -> Self {
        AsyncUnblocker {
            sender: Arc::new(Mutex::new(sender)),
        }
    }
}

impl Unblocker for AsyncUnblocker {
    fn unblock(&self, result: BlockResult) -> BlockResult {
        self.sender
            .lock()
            .as_mut()
            .map_err(|_| LogFSError::FailedToUnblock)?
            .try_send(result)
            .map_err(|_| LogFSError::FailedToUnblock)?;

        Ok(())
    }
}
