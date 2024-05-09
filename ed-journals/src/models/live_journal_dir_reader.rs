use crate::models::journal_dir::JournalDirError;
use crate::models::journal_file::JournalFileError;
use crate::models::journal_file_reader::JournalReaderError;
use crate::{JournalDir, JournalEvent, JournalFile, JournalFileReader};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::Thread;
use thiserror::Error;

/// Watches the whole journal dir and reads all files. Once all historic files have been read it
/// will block the current thread until the newest log file is changed at which it will read the
/// active log file and return the entry.
///
/// ```no_run
/// use std::path::PathBuf;
/// use ed_journals::LiveJournalDirReader;
///
/// let path = PathBuf::from("somePath")
///     .unwrap();
///
/// let live_dir_reader = LiveJournalDirReader::new(path)
///     .unwrap();
///
/// // At first this will read all existing lines from the journal logs, after which it will block
/// // the current thread until it detects new entries in the latest log file.
/// for entry in live_dir_reader {
///     // Do something with the entry
/// }
/// ```
#[derive(Debug)]
pub struct LiveJournalDirReader {
    waiting_thread: Arc<Mutex<(Option<Thread>,)>>,
    dir: JournalDir,
    current_file: Option<JournalFile>,
    current_reader: Option<JournalFileReader<File>>,
    watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
    failing: bool,
}

#[derive(Debug, Error)]
pub enum LiveJournalDirReaderError {
    #[error("Log path is not a directory")]
    PathIsNotADirectory,

    #[error(transparent)]
    JournalFileError(#[from] JournalFileError),

    #[error(transparent)]
    JournalReaderError(#[from] JournalReaderError),

    #[error(transparent)]
    JournalDirError(#[from] JournalDirError),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl LiveJournalDirReader {
    pub fn new(dir_path: PathBuf) -> Result<LiveJournalDirReader, LiveJournalDirReaderError> {
        let waiting_thread = Arc::new(Mutex::new((None::<Thread>,)));
        let waiting_thread_local = waiting_thread.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            let guard = waiting_thread_local
                .lock()
                .expect("Should have been locked");

            if let Some(thread) = guard.0.as_ref() {
                thread.unpark();
            };
        })?;

        watcher.watch(&dir_path, RecursiveMode::NonRecursive)?;

        Ok(Self {
            waiting_thread,
            dir: JournalDir::new(dir_path)?,
            current_file: None,
            current_reader: None,
            active: Arc::new(AtomicBool::new(true)),
            watcher,
            failing: false,
        })
    }

    fn set_current_file(
        &mut self,
        journal_file: JournalFile,
    ) -> Result<(), LiveJournalDirReaderError> {
        self.current_reader = Some(journal_file.create_reader()?);
        self.current_file = Some(journal_file);

        Ok(())
    }

    fn set_next_file(&mut self) -> Result<(), LiveJournalDirReaderError> {
        let files = self.dir.journal_logs_oldest_first()?;

        for file in files {
            let Some(current) = &self.current_file else {
                self.set_current_file(file)?;
                return Ok(());
            };

            if &file > current {
                self.set_current_file(file)?;
            }
        }

        Ok(())
    }

    fn handle(&self) -> LiveJournalDirHandle {
        LiveJournalDirHandle {
            active: self.active.clone(),
            waiting_thread: self.waiting_thread.clone(),
        }
    }
}

pub struct LiveJournalDirHandle {
    active: Arc<AtomicBool>,
    waiting_thread: Arc<Mutex<(Option<Thread>,)>>,
}

impl LiveJournalDirHandle {
    pub fn close(&self) {
        self.active.swap(false, Ordering::Relaxed);
        let guard = self.waiting_thread.lock().expect("to have gotten a lock");

        if let Some(a) = guard.0.as_ref() {
            a.unpark();
        };
    }
}

impl Iterator for LiveJournalDirReader {
    type Item = Result<JournalEvent, LiveJournalDirReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.active.load(Ordering::Relaxed) || self.failing {
                return None;
            }

            let result = self.set_next_file();

            if let Err(error) = result {
                self.failing = true;
                return Some(Err(error));
            }

            let reader = self.current_reader.as_mut()?;

            let Some(result) = reader.next() else {
                {
                    let mut guard = self.waiting_thread.lock().expect("to have gotten a lock");

                    guard.0 = Some(thread::current());
                }

                thread::park();

                continue;
            };

            return Some(result.map_err(|e| e.into()));
        }
    }
}
