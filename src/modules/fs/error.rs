use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LogFSError {
    IO(#[from] std::io::Error),
    NotifyError(#[from] notify::Error),
    RecvError(#[from] std::sync::mpsc::RecvError),

    #[cfg(feature = "asynchronous")]
    #[error("Failed to block")]
    AsyncRecvError,

    #[error("Failed to unblock")]
    FailedToUnblock,

    #[error("RwLock was poisoned")]
    PoisonError,
}
