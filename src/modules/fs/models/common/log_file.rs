use crate::fs::{Blocker, FileWatcher, LogFSError, Unblocker};
use crate::io::{LogIOError, LogIter};
use crate::logs::LogEvent;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

/// Holds both a watcher and an iterator over for the given path. Calling [Iterator::next] on this
/// will call the inner iterator.
#[derive(Debug)]
pub struct LogFile<R = LogEvent>
where
    R: DeserializeOwned,
{
    watcher: FileWatcher,
    iter: LogIter<BufReader<File>, R>,
}

impl LogFile {
    pub fn new<P: AsRef<Path>>(
        path: P,
        blocker: impl Into<Arc<dyn Unblocker>>,
    ) -> Result<LogFile<LogEvent>, LogFSError> {
        LogFile::new_typed::<LogEvent, _>(path, blocker)
    }

    pub fn new_typed<R, P>(
        path: P,
        blocker: impl Into<Arc<dyn Unblocker>>,
    ) -> Result<LogFile<R>, LogFSError>
    where
        R: DeserializeOwned,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let watcher = FileWatcher::new(path, blocker)?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let iter = LogIter::from(reader);

        Ok(LogFile { watcher, iter })
    }
}

impl<R> Iterator for LogFile<R>
where
    R: DeserializeOwned,
{
    type Item = Result<R, LogIOError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
