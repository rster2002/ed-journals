use thiserror::Error;

/// Error for any errors that occur while working with the file system
#[derive(Debug, Error)]
#[error(transparent)]
pub enum LogFSError {
    IO(#[from] std::io::Error),
    NotifyError(#[from] notify::Error),
    RecvError(#[from] std::sync::mpsc::RecvError),
    SerdeJson(#[from] serde_json::Error),

    #[cfg(feature = "asynchronous")]
    #[error("Failed to block")]
    AsyncRecvError,

    #[error("Failed to unblock")]
    FailedToUnblock,

    #[error("RwLock was poisoned")]
    PoisonError,
}
