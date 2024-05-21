use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcher;
use std::fs;
use std::path::Path;
use thiserror::Error;

pub type ShipLockerFileWatcher = LiveJsonFileWatcher<ShipLocker>;
pub use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcherError as ShipLockerFileWatcherError;
use crate::ship_locker::ShipLocker;

pub fn read_ship_locker_file<P: AsRef<Path>>(
    path: P,
) -> Result<ShipLocker, ReadShipLockerFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Error)]
pub enum ReadShipLockerFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse ship locker file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use crate::ship_locker::blocking::read_ship_locker_file;
    use std::env::current_dir;

    #[test]
    fn backpack_file_is_parsed_correctly() {
        let path = current_dir()
            .unwrap()
            .join("test-files")
            .join("json")
            .join("ShipLocker.json");

        let result = read_ship_locker_file(path);

        dbg!(&result);

        assert!(result.is_ok());
    }
}
