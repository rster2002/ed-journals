use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcher;
pub use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcherError as StatusFileWatcherError;
use crate::status::Status;

pub type StatusFileWatcher = LiveJsonFileWatcher<Status>;

pub fn read_status_file<P: AsRef<Path>>(path: P) -> Result<Status, ReadStatusFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Error)]
pub enum ReadStatusFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse status file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use crate::status::Status;
    use crate::tests::test_root;

    #[test]
    fn status_1_file_is_parsed_correctly() {
        let file = test_root()
            .join("json")
            .join("Status1.json");

        let string_contents = read_to_string(&file).unwrap();

        let status = serde_json::from_str::<Status>(&string_contents);

        dbg!(&status);
        assert!(status.is_ok());
    }
}
