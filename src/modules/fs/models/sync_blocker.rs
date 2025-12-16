mod sync_unblocker;

use std::sync::mpsc::{Receiver, Sender};
use crate::fs::BlockResult;
use crate::fs::models::sync_blocker::sync_unblocker::SyncUnblocker;
use crate::fs::traits::blocker::Blocker;
use crate::fs::traits::unblocker::Unblocker;

/// Handle for blocking in sync code.
pub struct SyncBlocker {
    receiver: Receiver<BlockResult>,
    sender: Sender<BlockResult>,
}

impl SyncBlocker {
    /// Creates a new blocker.
    pub fn new() -> SyncBlocker {
        SyncBlocker::default()
    }

    /// Blocks the current thread until a registered caller unblocks it.
    pub fn block(&mut self) -> BlockResult {
        self.receiver.recv()?
    }
}

impl Default for SyncBlocker {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        SyncBlocker {
            sender,
            receiver,
        }
    }
}

impl Blocker for SyncBlocker {
    fn unblocker(&self) -> Box<dyn Unblocker> {
        Box::new(SyncUnblocker::new(self.sender.clone()))
    }
}