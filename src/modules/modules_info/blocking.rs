use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcher;
pub use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcherError as ModulesInfoFileWatcherError;
use crate::modules_info::models::modules_info::ModulesInfo;

pub type NavRouteFileWatcher = LiveJsonFileWatcher<ModulesInfo>;

pub fn read_modules_info_file<P: AsRef<Path>>(
    path: P,
) -> Result<ModulesInfo, ReadModulesInfoFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Error)]
pub enum ReadModulesInfoFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse modules info file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::modules_info::blocking::read_modules_info_file;

    #[test]
    fn modules_info_file_is_parsed_correctly() {
        let path = current_dir()
            .unwrap()
            .join("test-files")
            .join("json")
            .join("ModulesInfo.json");

        let result = read_modules_info_file(path);

        dbg!(&result);

        assert!(result.is_ok());
        assert!(result.unwrap().modules.len() >= 3);
    }
}
