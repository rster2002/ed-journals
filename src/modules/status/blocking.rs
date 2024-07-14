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
    fn none_status_file_is_parsed_correctly() {
        let file = test_root()
            .join("json")
            .join("StatusNone.json");

        let string_contents = read_to_string(&file).unwrap();

        let status = serde_json::from_str::<Status>(&string_contents);

        dbg!(&status);
        assert!(status.is_ok());
        assert!(status.unwrap().contents.is_none());
    }

    #[test]
    fn supercruise_status_file_is_parsed_correctly() {
        let file = test_root()
            .join("json")
            .join("StatusSupercruise.json");

        let string_contents = read_to_string(&file).unwrap();

        let status = serde_json::from_str::<Status>(&string_contents);

        dbg!(&status);

        let status = status.unwrap()
            .contents
            .unwrap();

        assert!(!status.flags.landed());
        assert!(!status.flags.landing_gear_down());
        assert!(status.planet_status.is_none());
    }

    #[test]
    fn landed_status_file_is_parsed_correctly() {
        let file = test_root()
            .join("json")
            .join("StatusLanded.json");

        let string_contents = read_to_string(&file).unwrap();

        let status = serde_json::from_str::<Status>(&string_contents);

        dbg!(&status);
        let status = status.unwrap()
            .contents
            .unwrap();

        assert!(!status.flags.in_srv());
        assert!(status.flags.landed());
        assert!(status.flags.landing_gear_down());
        assert!(status.planet_status.is_some());
        assert!(status.kind.is_ship_status());
    }

    #[test]
    fn srv_status_file_is_parsed_correctly() {
        let file = test_root()
            .join("json")
            .join("StatusSRV.json");

        let string_contents = read_to_string(&file).unwrap();

        let status = serde_json::from_str::<Status>(&string_contents);

        dbg!(&status);
        let status = status.unwrap()
            .contents
            .unwrap();

        assert!(status.flags.in_srv());
        assert!(status.planet_status.is_some());
        assert!(status.kind.is_ship_status());
    }

    #[test]
    fn on_foot_status_file_is_parsed_correctly() {
        let file = test_root()
            .join("json")
            .join("StatusOnFoot.json");

        let string_contents = read_to_string(&file).unwrap();

        let status = serde_json::from_str::<Status>(&string_contents);

        dbg!(&status);
        let status = status.unwrap()
            .contents
            .unwrap();

        assert!(status.flags2.on_foot());
        assert!(status.planet_status.is_some());
        assert!(status.kind.is_on_foot_status());
    }
}
