use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::{CreateKind, DataChange, ModifyKind};
use crate::io::{LogError, LogIter};

pub struct LogFileWatcher {
    receiver: Receiver<Result<(), LogError>>,
    _watcher: RecommendedWatcher,
}

impl LogFileWatcher {
    /// Starts watching the given path and creates an [unbounded] channel.
    pub fn open_unbounded<P: AsRef<Path>>(path: P) -> Result<LogFileWatcher, LogError> {
        let (sender, receiver) = unbounded();

        let mut watcher = LogFileWatcher::create_watcher(sender)?;
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LogFileWatcher {
            receiver,
            _watcher: watcher,
        })
    }

    /// Starts watching the given path and creates a [bounded] channel with the given capacity.
    pub fn open_bounded<P: AsRef<Path>>(path: P, capacity: usize) -> Result<Self, LogError> {
        let (sender, receiver) = bounded(capacity);

        let mut watcher = LogFileWatcher::create_watcher(sender)?;
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LogFileWatcher {
            receiver,
            _watcher: watcher,
        })
    }

    fn create_watcher(sender: Sender<Result<(), LogError>>) -> Result<RecommendedWatcher, LogError> {
        Ok(notify::recommended_watcher(move |event| {
            let event: notify::Event = match event {
                Ok(event) => event,
                Err(error) => {
                    let _ = sender.send(Err(LogError::NotifyError(error)));
                    return;
                },
            };

            #[cfg(target_family = "unix")]
            match event.kind {
                EventKind::Create(CreateKind::File)
                | EventKind::Modify(ModifyKind::Data(DataChange::Content)) => true,
                _ => return,
            };

            #[cfg(target_family = "windows")]
            match event.kind {
                EventKind::Create(CreateKind::Any) | EventKind::Modify(ModifyKind::Any) => true,
                _ => return,
            };

            let _ = sender.send(Ok(()));
        })?)
    }
}