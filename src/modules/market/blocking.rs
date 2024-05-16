use std::fs;
use std::path::Path;
use thiserror::Error;
use crate::modules::market::Market;
use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcher;

pub type MarketFileWatcher = LiveJsonFileWatcher<Market>;
pub use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcherError as MarketFileWatcherError;

pub fn read_market_file<P: AsRef<Path>>(path: P) -> Result<Market, ReadMarketFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Error)]
pub enum ReadMarketFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse market file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::modules::market::blocking::read_market_file;

    #[test]
    fn market_file_is_parsed_correctly() {
        let path = current_dir()
            .unwrap()
            .join("test-files")
            .join("json")
            .join("Market.json");

        let result = read_market_file(&path);

        dbg!(&result);

        assert!(result.is_ok());
        assert!(result.unwrap().items.len() >= 3);
    }
}
