use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;
use tokio::fs::File;
use tokio::sync::mpsc::{channel, Sender};
use crate::logs::{LogDir, LogDirError, LogFile};
use crate::logs::content::LogEvent;
use crate::logs::r#async::LogFileReader;
use crate::modules::logs::r#async::LogFileReaderError;
use crate::modules::logs::LogFileError;

#[derive(Debug)]
pub struct LiveLogDirReader {
    waiting_sender: Arc<Mutex<(Option<Sender<()>>,)>>,
    dir: LogDir,
    current_file: Option<LogFile>,
    current_reader: Option<LogFileReader>,
    watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
    failing: bool,
}

#[derive(Debug, Error)]
pub enum LiveLogDirReaderError {
    #[error("Log path is not a directory")]
    PathIsNotADirectory,

    #[error(transparent)]
    JournalFileError(#[from] LogFileError),

    #[error(transparent)]
    JournalReaderError(#[from] LogFileReaderError),

    #[error(transparent)]
    JournalDirError(#[from] LogDirError),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl LiveLogDirReader {
    pub fn open<P: AsRef<Path>>(dir_path: P) -> Result<LiveLogDirReader, LiveLogDirReaderError> {
        let waiting_sender = Arc::new(Mutex::new((None::<Sender<()>>,)));
        let waiting_sender_local = waiting_sender.clone();

        let mut watcher = notify::recommended_watcher(move |res| {
            let mut guard = waiting_sender_local
                .lock()
                .expect("Should have been locked");

            if let Some(sender) = guard.0.as_ref() {
                if sender.is_closed() {
                    return;
                }

                sender.blocking_send(())
                    .expect("Failed to send");

                guard.0 = None;
            }
        })?;

        watcher.watch(dir_path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LiveLogDirReader {
            waiting_sender,
            dir: LogDir::new(dir_path.as_ref().to_path_buf())?,
            current_file: None,
            current_reader: None,
            active: Arc::new(AtomicBool::new(true)),
            watcher,
            failing: false,
        })
    }

    async fn set_current_file(
        &mut self,
        journal_file: LogFile,
    ) -> Result<(), LiveLogDirReaderError> {
        self.current_reader = Some(journal_file.create_async_reader().await?);
        self.current_file = Some(journal_file);

        Ok(())
    }

    async fn set_next_file(&mut self) -> Result<(), LiveLogDirReaderError> {
        let files = self.dir.journal_logs_oldest_first()?;

        for file in files {
            let Some(current) = &self.current_file else {
                self.set_current_file(file)
                    .await?;
                return Ok(());
            };

            if &file > current {
                self.set_current_file(file)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn next(&mut self) -> Option<Result<LogEvent, LiveLogDirReaderError>> {
        loop {
            if !self.active.load(Ordering::Relaxed) || self.failing {
                return None;
            }

            let result = self.set_next_file()
                .await;

            if let Err(error) = result {
                self.failing = true;
                return Some(Err(error));
            }

            let reader = self.current_reader.as_mut()?;

            let Some(result) = reader.next().await else {
                let (sender, mut receiver) = channel(2);

                {
                    let mut guard = self.waiting_sender.lock()
                        .expect("to gotten lock");

                    guard.0 = Some(sender);
                }

                receiver.recv().await?;

                continue;
            };

            return Some(result.map_err(|e| e.into()));
        }
    }
}
