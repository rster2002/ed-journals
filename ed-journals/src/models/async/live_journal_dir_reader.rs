use std::io;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;
use tokio::fs::File;
use tokio::sync::mpsc::{channel, Sender};
use crate::r#async::journal_file_reader::JournalFileReaderError;
use crate::r#async::JournalFileReader;
use crate::{JournalDir, JournalDirError, JournalEvent, JournalFile, JournalFileError};

#[derive(Debug)]
pub struct LiveJournalDirReader {
    waiting_sender: Arc<Mutex<(Option<Sender<()>>,)>>,
    dir: JournalDir,
    current_file: Option<JournalFile>,
    current_reader: Option<JournalFileReader>,
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
    JournalReaderError(#[from] JournalFileReaderError),

    #[error(transparent)]
    JournalDirError(#[from] JournalDirError),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl LiveJournalDirReader {
    pub fn create(dir_path: PathBuf) -> Result<LiveJournalDirReader, LiveJournalDirReaderError> {
        let waiting_sender = Arc::new(Mutex::new((None::<Sender<()>>,)));
        let waiting_sender_local = waiting_sender.clone();

        let mut watcher = notify::recommended_watcher(move |res| {
            let guard = waiting_sender_local
                .lock()
                .expect("Should have been locked");

            if let Some(sender) = guard.0.as_ref() {
                if sender.is_closed() {
                    return;
                }

                dbg!(sender.blocking_send(()))
                    .expect("Failed to send");
            }
        })?;

        watcher.watch(&dir_path, RecursiveMode::NonRecursive)?;

        Ok(LiveJournalDirReader {
            waiting_sender,
            dir: JournalDir::new(dir_path)?,
            current_file: None,
            current_reader: None,
            active: Arc::new(AtomicBool::new(true)),
            watcher,
            failing: false,
        })
    }

    async fn set_current_file(
        &mut self,
        journal_file: JournalFile,
    ) -> Result<(), LiveJournalDirReaderError> {
        self.current_reader = Some(journal_file.create_async_reader().await?);
        self.current_file = Some(journal_file);

        Ok(())
    }

    async fn set_next_file(&mut self) -> Result<(), LiveJournalDirReaderError> {
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

    pub async fn next(&mut self) -> Option<Result<JournalEvent, LiveJournalDirReaderError>> {
        loop {
            // if !self.active.load(Ordering::Relaxed) || self.failing {
            //     return None;
            // }

            let (sender, mut receiver) = channel(2);

            {
                let mut guard = dbg!(self.waiting_sender.lock())
                    .expect("to gotten lock");

                guard.0 = Some(sender);
            }

            receiver.recv().await?;

            dbg!("Hi");

            // let result = self.set_next_file()
            //     .await;
            //
            // if let Err(error) = result {
            //     self.failing = true;
            //     return Some(Err(error));
            // }
            //
            // let reader = self.current_reader.as_mut()?;
            //
            // let Some(result) = reader.next().await else {
            //     let (sender, mut receiver) = channel(2);
            //
            //     {
            //         let mut guard = dbg!(self.waiting_sender.lock())
            //             .expect("to gotten lock");
            //
            //         guard.0 = Some(sender);
            //     }
            //
            //     receiver.recv().await?;
            //
            //     continue;
            // };
            //
            // return Some(result.map_err(|e| e.into()));
        }
    }
}
