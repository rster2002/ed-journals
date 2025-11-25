use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
// use futures::SinkExt;
// use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::{CreateKind, DataChange, ModifyKind};
use crate::io::{LogError, LogIter};

pub struct LogFileWatcher {
    // Oh god
    sender: Arc<RwLock<Option<Sender<Result<(), LogError>>>>>,
    _watcher: RecommendedWatcher,
}

impl LogFileWatcher {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<LogFileWatcher, LogError> {
        let sender = Arc::new(RwLock::new(None::<Sender<Result<(), LogError>>>));
        let local_senders = sender.clone();

        let mut watcher = notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
            dbg!(&event);

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
        dbg!("Start watching here");
        dbg!(path.as_ref());

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

    // /// Starts watching the given path and creates an [unbounded] channel.
    // pub fn open_unbounded<P: AsRef<Path>>(path: P) -> Result<LogFileWatcher, LogError> {
    //     let (sender, receiver) = unbounded();
    //
    //     let mut watcher = LogFileWatcher::create_watcher(sender)?;
    //     watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    //
    //     Ok(LogFileWatcher {
    //         receiver,
    //         _watcher: watcher,
    //     })
    // }
    //
    // /// Starts watching the given path and creates a [bounded] channel with the given capacity.
    // pub fn open_bounded<P: AsRef<Path>>(path: P, capacity: usize) -> Result<Self, LogError> {
    //     let (sender, receiver) = bounded(capacity);
    //
    //     let mut watcher = LogFileWatcher::create_watcher(sender)?;
    //     watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    //
    //     Ok(LogFileWatcher {
    //         receiver,
    //         _watcher: watcher,
    //     })
    // }
    //
    // pub fn next(&self) -> Result<(), LogError> {
    //     self.receiver.recv()?
    // }

    // fn create_watcher(sender: Sender<Result<(), LogError>>) -> Result<RecommendedWatcher, LogError> {
    //     Ok(notify::recommended_watcher(move |event| {
    //         let event: notify::Event = match event {
    //             Ok(event) => event,
    //             Err(error) => {
    //                 let _ = sender.send(Err(LogError::NotifyError(error)));
    //                 return;
    //             },
    //         };
    //
    //         #[cfg(target_family = "unix")]
    //         match event.kind {
    //             EventKind::Create(CreateKind::File)
    //             | EventKind::Modify(ModifyKind::Data(_)) => true,
    //             _ => return,
    //         };
    //
    //         #[cfg(target_family = "windows")]
    //         match event.kind {
    //             EventKind::Create(CreateKind::Any) | EventKind::Modify(ModifyKind::Any) => true,
    //             _ => return,
    //         };
    //
    //         let _ = sender.send(Ok(()));
    //     })?)
    // }
}

#[cfg(test)]
mod tests {
    // use crate::io::LogFileWatcher;
    // use crate::modules::tests::simulate_log_file;
    //
    // #[test]
    // fn basic_watching_works_as_expected() {
    //     let (path, handle) = simulate_log_file("basic_log.log");
    //     let watcher = LogFileWatcher::open_bounded(&path, 10).unwrap();
    //
    //     watcher.next().unwrap();
    // }
}