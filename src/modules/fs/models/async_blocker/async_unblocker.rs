use futures::channel::mpsc::Sender;
use futures::SinkExt;
use crate::fs::{BlockResult, LogFSError};
use crate::fs::traits::unblocker::Unblocker;

pub struct AsyncUnblocker {
    sender: Sender<BlockResult>,
}

impl AsyncUnblocker {
    pub fn new(sender: Sender<BlockResult>) -> Self {
        AsyncUnblocker {
            sender,
        }
    }
}

impl Unblocker for AsyncUnblocker {
    fn unblock(&mut self, result: BlockResult) -> BlockResult {
        self.sender.try_send(result)
            .map_err(|_| LogFSError::FailedToUnblock)?;

        Ok(())
    }
}