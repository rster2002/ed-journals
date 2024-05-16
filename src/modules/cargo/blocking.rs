use std::fs;
use std::path::Path;
use thiserror::Error;
use crate::backpack::models::backpack::Backpack;
use crate::modules::cargo::models::cargo::Cargo;
use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcher;

pub type CargoFileWatcher = LiveJsonFileWatcher<Cargo>;
pub use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcherError as CargoFileWatcherError;

pub fn read_cargo_file<P: AsRef<Path>>(path: P) -> Result<Cargo, ReadCargoFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Error)]
pub enum ReadCargoFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse cargo file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::modules::cargo::blocking::read_cargo_file;

    #[test]
    fn cargo_file_is_parsed_correctly() {
        let path = current_dir()
            .unwrap()
            .join("test-files")
            .join("json")
            .join("Cargo.json");

        let result = read_cargo_file(&path);

        dbg!(&result);

        assert!(result.is_ok());
    }
}
