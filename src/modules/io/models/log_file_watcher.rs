use crate::io::LogError;
use notify::event::{CreateKind, ModifyKind};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};

pub struct LogFileWatcher {
    sender: Arc<RwLock<Option<Sender<Result<(), LogError>>>>>,
    _watcher: RecommendedWatcher,
}

impl LogFileWatcher {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<LogFileWatcher, LogError> {
        let sender = Arc::new(RwLock::new(None::<Sender<Result<(), LogError>>>));
        let local_senders = sender.clone();

        let mut watcher = notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
            let sender_lock = local_senders.read()
                .expect("Failed to get rw lock");

            let Some(sender) = sender_lock.as_ref() else {
                return;
            };

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
                | EventKind::Modify(ModifyKind::Data(_)) => true,
                _ => return,
            };

            #[cfg(target_family = "windows")]
            match event.kind {
                EventKind::Create(CreateKind::Any) | EventKind::Modify(ModifyKind::Any) => true,
                _ => return,
            };

            let _ = sender.send(Ok(()));
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LogFileWatcher {
            sender,
            _watcher: watcher,
        })
    }

    pub fn set_sender(&self, sender: Sender<Result<(), LogError>>) -> Result<(), LogError> {
        let mut guard = self.sender.write()
            .map_err(|_| LogError::PoisonError)?;

        *guard = Some(sender);

        Ok(())
    }
}