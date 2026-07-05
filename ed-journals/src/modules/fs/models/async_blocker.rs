pub mod async_unblocker;

use crate::fs::models::async_blocker::async_unblocker::AsyncUnblocker;
use crate::fs::traits::blocker::Blocker;
use crate::fs::traits::unblocker::Unblocker;
use crate::fs::{BlockResult, LogFSError};
use futures::channel::mpsc::{Receiver, Sender};
use futures::StreamExt;
use std::sync::Arc;

/// Blocker that can be used to block in async code.
pub struct AsyncBlocker {
    receiver: Receiver<BlockResult>,
    sender: Sender<BlockResult>,
}

impl AsyncBlocker {
    /// Create a new AsyncBlocker with the given capacity.
    pub fn new(capacity: usize) -> AsyncBlocker {
        let (sender, receiver) = futures::channel::mpsc::channel(capacity);

        AsyncBlocker { sender, receiver }
    }

    /// Block and await the current task until a registered caller unblocks it.
    pub async fn wait(&mut self) -> BlockResult {
        self.receiver
            .next()
            .await
            .ok_or(LogFSError::AsyncRecvError)?
    }
}

impl Blocker for AsyncBlocker {
    fn unblocker(&self) -> Arc<dyn Unblocker> {
        Arc::new(AsyncUnblocker::new(self.sender.clone()))
    }
}

impl From<AsyncBlocker> for Arc<dyn Unblocker> {
    fn from(blocker: AsyncBlocker) -> Arc<dyn Unblocker> {
        blocker.unblocker()
    }
}

impl From<&AsyncBlocker> for Arc<dyn Unblocker> {
    fn from(blocker: &AsyncBlocker) -> Arc<dyn Unblocker> {
        blocker.unblocker()
    }
}
