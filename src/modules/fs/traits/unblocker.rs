use crate::fs::BlockResult;

/// Mechanism that is given to some watcher to unblock the associated blocker. Should always be
/// callable in sync code.
pub trait Unblocker: Send {
    /// Unblock the associated blocker.
    fn unblock(&mut self, result: BlockResult) -> BlockResult;
}
