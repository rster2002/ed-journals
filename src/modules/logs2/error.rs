use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LogError {
    IO(#[from] std::io::Error),
    SerdeJson(#[from] serde_json::Error),
}