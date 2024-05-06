use std::collections::VecDeque;
use std::fs::File;
use std::path::PathBuf;
use std::{io, thread};
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{JoinHandle, Thread};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use notify::RecursiveMode::Recursive;
use thiserror::Error;
use crate::{JournalEvent, JournalFileReader};
use crate::journal_event_content::carrier_crew_services_event::CarrierCrewServicesEventOperation::Activate;
use crate::models::journal_file_reader::JournalReaderError;

#[derive(Debug)]
pub struct LiveJournalFileReader {
    waiting_thread: Arc<Mutex<(Option<Thread>,)>>,
    entry_buffer: VecDeque<JournalEvent>,
    journal_file_reader: JournalFileReader<File>,
    watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
    // waiting_threads: Vec<Thread>,
}

#[derive(Debug, Error)]
pub enum LiveJournalFileReaderError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl LiveJournalFileReader {
    pub fn new(path: PathBuf) -> Result<Self, LiveJournalFileReaderError> {
        let file = File::open(&path)?;
        let journal_file_reader = JournalFileReader::from(file);

        let waiting_thread = Arc::new(Mutex::new((None::<Thread>,)));
        let waiting_thread_local = waiting_thread.clone();

        let mut watcher = notify::recommended_watcher(move |res| {
            let guard = waiting_thread_local.lock().unwrap();

            if let Some(a) = guard.0.as_ref() {
                a.unpark();
            };
        })?;

        watcher.watch(&path, RecursiveMode::NonRecursive)?;

        Ok(LiveJournalFileReader {
            waiting_thread,
            entry_buffer: VecDeque::new(),
            journal_file_reader,
            watcher,
            active: Arc::new(AtomicBool::new(true)),
        })
    }

    pub fn activation(&self) -> JournalHandle {
        JournalHandle::new(self.active.clone(), self.waiting_thread.clone())
    }
}

pub struct JournalHandle {
    active: Arc<AtomicBool>,
    waiting_thread: Arc<Mutex<(Option<Thread>,)>>,
}

impl JournalHandle {
    pub fn new(active: Arc<AtomicBool>, waiting_thread: Arc<Mutex<(Option<Thread>,)>>) -> JournalHandle {
        JournalHandle {
            active,
            waiting_thread,
        }
    }

    pub fn close(&self) {
        self.active.swap(false, Ordering::Relaxed);
        let guard = self.waiting_thread.lock()
            .unwrap();

        if let Some(a) = guard.0.as_ref() {
            a.unpark();
        };
    }
}

impl Iterator for LiveJournalFileReader {
    type Item = Result<JournalEvent, JournalReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.active.load(Ordering::Relaxed) {
                return None;
            }

            match self.journal_file_reader.next() {
                Some(value) => return Some(value),
                None => {
                    {
                        let mut guard = self.waiting_thread.lock()
                            .unwrap();

                        guard.0 = Some(thread::current());
                    }

                    thread::park();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::atomic::Ordering;
    use std::thread;
    use crate::JournalEventContentKind;
    use crate::models::live_journal_file_reader::LiveJournalFileReader;

    #[test]
    fn live_reader_reads_journal_file_correctly() {
        let mut file = File::create("input.log")
            .unwrap();

        file.write_all(r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#.as_bytes())
            .unwrap();

        let reader = LiveJournalFileReader::new(PathBuf::from("input.log"))
            .unwrap();

        let active = reader.activation();
        let active_local = reader.activation();

        let handle = thread::spawn(move || {
            let mut count = 0;

            for entry in reader {
                let entry = entry.unwrap();
                count += 1;

                if count == 1 {
                    assert_eq!(entry.content.kind(), JournalEventContentKind::FileHeader);

                    file.write_all(r#"{"timestamp":"2022-10-22T15:12:05Z","event":"Commander","FID":"F123456789","Name":"TEST"}"#.as_bytes())
                        .unwrap();
                }

                if count == 2 {
                    assert_eq!(entry.content.kind(), JournalEventContentKind::Commander);

                    active_local.close();
                }
            }
        });

        handle.join().unwrap();
    }
}
