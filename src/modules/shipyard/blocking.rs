use std::fs;
use std::path::Path;
use crate::modules::shipyard::models::shipyard::Shipyard;
use thiserror::Error;
use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcher;

pub type ShipyardFileWatcher = LiveJsonFileWatcher<Shipyard>;
pub use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcherError as ShipyardFileWatcherError;

pub fn read_shipyard_file<P: AsRef<Path>>(path: P) -> Result<Shipyard, ReadShipyardFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Error)]
pub enum ReadShipyardFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse shipyard file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::modules::shipyard::blocking::read_shipyard_file;

    #[test]
    fn shipyard_file_is_parsed_correctly() {
        let path = current_dir()
            .unwrap()
            .join("test-files")
            .join("json")
            .join("Shipyard.json");

        let result = read_shipyard_file(path);

        dbg!(&result);

        assert!(result.is_ok());
        assert!(result.unwrap().price_list.len() > 10);
    }
}
