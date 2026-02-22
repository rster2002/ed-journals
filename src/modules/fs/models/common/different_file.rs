use crate::fs::common::LogFile;
use crate::io::LogPath;

#[derive(Debug, Default)]
pub struct DifferentFile {
    // current_file: Option<LogFile>,
}

impl DifferentFile {
    pub fn new() -> DifferentFile {
        DifferentFile::default()
    }

    pub fn maybe_switch(&mut self, path: &LogPath) {}
}
