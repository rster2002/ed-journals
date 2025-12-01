use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LogFSError {
    IO(#[from] std::io::Error),
    NotifyError(#[from] notify::Error),

    #[error("RwLock was poisoned")]
    PoisonError,
}