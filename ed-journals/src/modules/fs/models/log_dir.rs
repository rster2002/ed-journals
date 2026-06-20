use crate::fs::error::LogFSError;
use crate::io::LogPath;
use std::fs;
use std::path::{Path, PathBuf};

/// Reads the contents of the target directory each time [Iterator::next] is called and returns
/// the next file in the directory.
pub struct LogDir {
    path: PathBuf,
    files: Vec<LogPath>,
    index: usize,
}

impl LogDir {
    /// Creates a new [LogDir] instance targeting the given path. The provided path must point to
    /// a directory.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        LogDir {
            path: path.into(),
            index: 0,
            files: Vec::new(),
        }
    }

    /// Return the last `n` files after which the newest will be returned when available.
    pub fn last_n(&mut self, n: usize) -> Option<Result<LogPath, LogFSError>> {
        if let Err(err) = self.read_dir() {
            return Some(Err(err));
        }

        let target_index = self.files.len().saturating_sub(n);
        if self.index < target_index {
            self.index = target_index;
        }

        self.inner_next().transpose()
    }

    /// Whether the given path is the last file in the directory.
    pub fn is_last(&self, path: &LogPath) -> bool {
        self.files.last().is_some_and(|last| last == path)
    }

    fn read_dir(&mut self) -> Result<(), LogFSError> {
        let items = fs::read_dir(&self.path)?;

        // Allocate at least the current len, as most of the time there are at least that many files
        // still in the directory.
        let mut new_files = Vec::with_capacity(self.files.len());

        for item in items {
            let item = item?;

            let path = item.path();

            let Ok(log_path) = LogPath::try_from(path.as_ref()) else {
                continue;
            };

            new_files.push(log_path);
        }

        new_files.sort();
        self.files = new_files;

        Ok(())
    }

    fn inner_next(&mut self) -> Result<Option<LogPath>, LogFSError> {
        self.read_dir()?;

        if self.index >= self.files.len() {
            self.index = self.files.len().saturating_sub(1);
            return Ok(None);
        }

        let result = self.files.get(self.index).cloned();
        self.index += 1;

        Ok(result)
    }
}

impl Iterator for LogDir {
    type Item = Result<LogPath, LogFSError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_next().transpose()
    }
}

impl AsRef<Path> for LogDir {
    fn as_ref(&self) -> &Path {
        self.path.as_path()
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::fs::models::log_dir::LogDir;
    use crate::tests::{test_dir, test_root};
    use std::fs;

    #[test]
    fn entries_are_returned_correctly() {
        let mut log_dir = LogDir::new(test_root().join("journals"));

        let entry = log_dir.next();
        assert!(entry.is_some());
        assert!(dbg!(entry.unwrap()).is_ok());
    }

    #[test]
    fn entries_end_correctly() {
        let dir = test_dir();
        fs::write(dir.path().join("Journal.2022-11-01T182054.01.log"), "").unwrap();

        let mut log_dir = LogDir::new(dir.path());

        log_dir.next().unwrap().unwrap();
        assert!(log_dir.next().is_none());
    }

    #[test]
    fn new_entries_are_read_correctly_after_reaching_the_end() {
        let dir = test_dir();
        fs::write(dir.path().join("Journal.2022-11-01T182054.01.log"), "").unwrap();

        let mut log_dir = LogDir::new(dir.path());

        assert!(log_dir.next().is_some());
        assert!(log_dir.next().is_none());

        fs::write(dir.path().join("Journal.2022-11-01T182054.02.log"), "").unwrap();

        assert!(log_dir.next().is_some());
    }

    #[test]
    fn recreated_entries_are_read_correctly() {
        let dir = test_dir();
        let second_file = dir.path().join("Journal.2022-11-01T182054.02.log");

        fs::write(dir.path().join("Journal.2022-11-01T182054.01.log"), "").unwrap();
        fs::write(&second_file, "").unwrap();

        let mut log_dir = LogDir::new(dir.path());

        assert!(log_dir.next().is_some());
        assert!(log_dir.next().is_some());
        assert!(log_dir.next().is_none());

        fs::remove_file(&second_file).unwrap();

        assert!(log_dir.next().is_none());

        fs::write(&second_file, "").unwrap();

        assert!(log_dir.next().is_some());
    }

    #[test]
    fn last_n_are_returned_correctly() {
        let dir = test_dir();
        fs::write(dir.path().join("Journal.2022-11-01T182054.01.log"), "").unwrap();
        fs::write(dir.path().join("Journal.2022-11-01T182054.02.log"), "").unwrap();
        fs::write(dir.path().join("Journal.2022-11-01T182054.03.log"), "").unwrap();
        fs::write(dir.path().join("Journal.2022-11-01T182054.04.log"), "").unwrap();
        fs::write(dir.path().join("Journal.2022-11-01T182054.05.log"), "").unwrap();

        let mut log_dir = LogDir::new(dir.path());

        let last = log_dir.last_n(3).unwrap().unwrap();
        assert_eq!(last.part, 3);

        let last = log_dir.last_n(3).unwrap().unwrap();
        assert_eq!(last.part, 4);

        let last = log_dir.last_n(3).unwrap().unwrap();
        assert_eq!(last.part, 5);

        assert!(log_dir.last_n(3).is_none());
    }
}
