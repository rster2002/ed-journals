use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::backpack::models::backpack::Backpack;
use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcher;
pub use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcherError as BackpackFileWatcherError;

pub type BackpackFileWatcher = LiveJsonFileWatcher<Backpack>;

/// Read the contents of the 'Backpack.json' file at the given path.
pub fn read_backpack_file<P: AsRef<Path>>(path: P) -> Result<Backpack, ReadBackpackFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Error)]
pub enum ReadBackpackFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse backpack file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::backpack::blocking::read_backpack_file;

    #[test]
    fn backpack_file_is_parsed_correctly() {
        let path = current_dir()
            .unwrap()
            .join("test-files")
            .join("json")
            .join("Backpack.json");

        let result = read_backpack_file(path);

        dbg!(&result);

        assert!(result.is_ok());
    }
}
