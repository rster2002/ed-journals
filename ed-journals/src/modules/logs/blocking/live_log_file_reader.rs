use std::collections::VecDeque;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::{io, thread};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;
use crate::logs::content::LogEvent;
use crate::modules::logs::blocking::{LogFileReader, LogFileReaderError};

/// Allows you to iterate over a journal log file and blocks when there are no entries to read, then
/// when the file changes it will unblock and return the new line(s).
///
/// ```no_run
/// use std::path::PathBuf;
/// use ed_journals::logs::blocking::LiveLogFileReader;
///
/// let path = PathBuf::from("somePath");
///
/// let live_reader = LiveLogFileReader::new(path)
///     .unwrap();
///
/// // This will block the current thread until there are new entries.
/// for entry in live_reader {
///     // Do something with the entry
/// }
/// ```
#[derive(Debug)]
pub struct LiveLogFileReader {
    waiting_thread: Arc<Mutex<(Option<Thread>,)>>,
    log_file_reader: LogFileReader,
    watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
}

#[derive(Debug, Error)]
pub enum LiveLogFileReaderError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),

    #[error(transparent)]
    LogFileReaderError(#[from] LogFileReaderError),
}

impl LiveLogFileReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, LiveLogFileReaderError> {
        let journal_file_reader = LogFileReader::open(&path)?;

        let waiting_thread = Arc::new(Mutex::new((None::<Thread>,)));
        let waiting_thread_local = waiting_thread.clone();

        // This is stopped when it is dropped
        let mut watcher = notify::recommended_watcher(move |_| {
            let mut guard = waiting_thread_local
                .lock()
                .expect("Should have been locked");

            if let Some(thread) = guard.0.as_ref() {
                thread.unpark();
                guard.0 = None;
            };
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LiveLogFileReader {
            waiting_thread,
            log_file_reader: journal_file_reader,
            watcher,
            active: Arc::new(AtomicBool::new(true)),
        })
    }

    pub fn handle(&self) -> LiveLogFileHandle {
        LiveLogFileHandle {
            active: self.active.clone(),
            waiting_thread: self.waiting_thread.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiveLogFileHandle {
    active: Arc<AtomicBool>,
    waiting_thread: Arc<Mutex<(Option<Thread>,)>>,
}

impl LiveLogFileHandle {
    pub fn stop(&self) {
        self.active.swap(false, Ordering::Relaxed);
        let guard = self.waiting_thread.lock().expect("to have gotten a lock");

        if let Some(thread) = guard.0.as_ref() {
            thread.unpark();
        };
    }
}

impl Iterator for LiveLogFileReader {
    type Item = Result<LogEvent, LogFileReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.active.load(Ordering::Relaxed) {
                return None;
            }

            match self.log_file_reader.next() {
                Some(value) => return Some(value),
                None => {
                    {
                        let mut guard = self.waiting_thread.lock().expect("to have gotten a lock");

                        guard.0 = Some(thread::current());
                    }

                    thread::park();
                }
            }
        }
    }
}
