use std::fs;
use std::path::Path;
use thiserror::Error;
use crate::modules::nav_route::models::nav_route::NavRoute;
use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcher;

pub type NavRouteFileWatcher = LiveJsonFileWatcher<NavRoute>;
pub use crate::modules::shared::blocking::live_json_file_watcher::LiveJsonFileWatcherError as NavRouteFileWatcherError;

pub fn read_nav_route_file<P: AsRef<Path>>(path: P) -> Result<NavRoute, ReadNavRouteFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Error)]
pub enum ReadNavRouteFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse nav route file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::modules::nav_route::blocking::read_nav_route_file;

    #[test]
    fn nav_route_file_is_parsed_correctly() {
        let path = current_dir()
            .unwrap()
            .join("test-files")
            .join("json")
            .join("NavRoute.json");

        let result = read_nav_route_file(&path);

        dbg!(&result);

        assert!(result.is_ok());
        assert!(result.unwrap().route.len() >= 3);
    }
}
