use crate::fs::BlockResult;

/// Mechanism that is given to some watcher to unblock the associated blocker. Should always be
/// callable in sync code.
pub trait Unblocker: Send + Sync {
    /// Unblock the associated blocker.
    fn unblock(&self, result: BlockResult) -> BlockResult;
}
