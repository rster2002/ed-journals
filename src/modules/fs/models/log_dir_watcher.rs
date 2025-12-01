use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::{CreateKind, ModifyKind, RemoveKind};
use crate::fs::error::LogFSError;

pub struct LogDirWatcher {
    sender: Arc<RwLock<Option<Sender<Result<(), LogFSError>>>>>,
    _watcher: RecommendedWatcher,
}

impl LogDirWatcher {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<LogDirWatcher, LogFSError> {
        let senders = Arc::new(RwLock::new(None::<Sender<Result<(), LogFSError>>>));
        let local_senders = senders.clone();

        let mut watcher = notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
            let sender_lock = local_senders.read()
                .expect("Failed to get rw lock");

            let Some(sender) = sender_lock.as_ref() else {
                return;
            };

            let event: notify::Event = match event {
                Ok(event) => event,
                Err(error) => {
                    let _ = sender.send(Err(LogFSError::NotifyError(error)));
                    return;
                },
            };

            #[cfg(target_family = "unix")]
            match event.kind {
                EventKind::Create(CreateKind::File)
                | EventKind::Remove(RemoveKind::Any) => true,
                _ => return,
            };

            #[cfg(target_family = "windows")]
            match event.kind {
                EventKind::Create(CreateKind::Any) | EventKind::Remove(RemoveKind::Any) => true,
                _ => return,
            };

            let _ = sender.send(Ok(()));
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LogDirWatcher {
            sender: senders,
            _watcher: watcher,
        })
    }

    pub fn set_sender(&self, sender: Sender<Result<(), LogFSError>>) -> Result<(), LogFSError> {
        let mut guard = self.sender.write()
            .map_err(|_| LogFSError::PoisonError)?;

        *guard = Some(sender);

        Ok(())
    }
}