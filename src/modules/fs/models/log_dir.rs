use std::fs;
use std::path::PathBuf;
use crate::fs::error::LogFSError;
use crate::io::LogPath;

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

    /// Skip the read index to the current last entry that is available and returns it, or [None] if
    /// there isn't an entry to return. No matter how many times this is called this will always
    /// return the last entry.
    pub fn skip_to_last(&mut self) -> Option<Result<LogPath, LogFSError>> {
        self.inner_skip_to_last().transpose()
    }

    fn inner_skip_to_last(&mut self) -> Result<Option<LogPath>, LogFSError> {
        self.read_dir()?;
        self.index = self.files.len().saturating_sub(1);

        let value = self.files.get(self.index).cloned();
        self.index += 1;

        Ok(value)
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

            new_files.push(LogPath::from(log_path));
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

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::modules::fs::models::log_dir::LogDir;
    use crate::tests::test_dir;

    #[test]
    fn entries_are_returned_correctly() {
        let mut log_dir = LogDir::new("./test-files/journals");

        let entry = log_dir.next();
        assert!(entry.is_some());
        assert!(entry.unwrap().is_ok());
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
    fn skip_to_last_always_returns_the_last_entry() {
        let dir = test_dir();
        fs::write(dir.path().join("Journal.2022-11-01T182054.01.log"), "").unwrap();
        fs::write(dir.path().join("Journal.2022-11-01T182054.02.log"), "").unwrap();

        let mut log_dir = LogDir::new(dir.path());

        let last = log_dir.skip_to_last().unwrap().unwrap();
        assert_eq!(last.part, 2);
        assert!(log_dir.next().is_none());

        assert!(log_dir.skip_to_last().is_some());
        assert!(log_dir.skip_to_last().is_some());
    }
}