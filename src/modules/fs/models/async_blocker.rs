mod async_unblocker;

use futures::channel::mpsc::{Receiver, Sender};
use futures::StreamExt;
use crate::fs::{BlockResult, LogFSError};
use crate::fs::models::async_blocker::async_unblocker::AsyncUnblocker;
use crate::fs::traits::blocker::Blocker;
use crate::fs::traits::unblocker::Unblocker;

/// Blocker that can be used to block in async code.
pub struct AsyncBlocker {
    receiver: Receiver<BlockResult>,
    sender: Sender<BlockResult>,
}

impl AsyncBlocker {
    /// Block and await the current task until a registered caller unblocks it.
    async fn block(&mut self) -> BlockResult {
        self.receiver.next()
            .await
            .ok_or(LogFSError::AsyncRecvError)?
    }
}

impl Blocker for AsyncBlocker {
    fn unblocker(&self) -> Box<dyn Unblocker> {
        Box::new(AsyncUnblocker::new(self.sender.clone()))
    }
}