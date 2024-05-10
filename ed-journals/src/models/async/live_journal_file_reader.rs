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
use crate::JournalEvent;

#[derive(Debug)]
pub struct LiveJournalFileReader {
    waiting_sender: Arc<Mutex<(Option<Sender<()>>,)>>,
    journal_file_reader: JournalFileReader,
    watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
}

#[derive(Debug, Error)]
pub enum LiveJournalFileReaderError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl LiveJournalFileReader {
    pub async fn create(path: PathBuf) -> Result<Self, LiveJournalFileReaderError> {
        let file = File::open(&path)
            .await?;

        let journal_file_reader = JournalFileReader::new(file);

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

                sender.blocking_send(())
                    .expect("Failed to send");
            }
        })?;

        watcher.watch(&path, RecursiveMode::NonRecursive)?;

        Ok(LiveJournalFileReader {
            waiting_sender,
            journal_file_reader,
            watcher,
            active: Arc::new(AtomicBool::new(true)),
        })
    }

    pub async fn next(&mut self) -> Option<Result<JournalEvent, JournalFileReaderError>> {
        loop {
            if !self.active.load(Ordering::Relaxed) {
                return None;
            }

            match self.journal_file_reader.next().await {
                Some(value) => return Some(value),
                None => {
                    let (sender, mut receiver) = channel(2);

                    {
                        let mut guard = dbg!(self.waiting_sender.lock())
                            .expect("to gotten lock");

                        guard.0 = Some(sender);
                    }

                    receiver.recv().await?;
                }
            }
        }
    }
}
