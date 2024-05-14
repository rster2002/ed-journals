use std::fs;
use std::path::Path;
use thiserror::Error;
use crate::modules::outfitting::Outfitting;
use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcher;

pub type StatusFileWatcher = LiveJsonFileWatcher<Outfitting>;
pub use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcherError as StatusFileWatcherError;

pub fn read_outfitting_file<P: AsRef<Path>>(path: P) -> Result<Outfitting, ReadOutfittingFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Error)]
pub enum ReadOutfittingFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse status file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::modules::outfitting::blocking::read_outfitting_file;

    #[test]
    fn outfitting_file_is_parsed_correctly() {
        let path = current_dir()
            .unwrap()
            .join("test-files")
            .join("json")
            .join("Outfitting.json");

        let result = read_outfitting_file(&path);

        dbg!(&result);

        assert!(result.is_ok());
        assert!(result.unwrap().items.len() > 10);
    }
}
