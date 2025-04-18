use crate::io::{DirIter, LogError, LogFile, LogPath};
use crate::modules::shared::asynchronous::async_blocker::AsyncBlocker;
use futures::{FutureExt, Stream};
use notify::event::CreateKind;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::VecDeque;
use std::path::Path;
use std::pin::{pin, Pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

#[derive(Debug)]
pub struct AsyncLiveDirIter {
    dir_iter: DirIter,
    blocker: AsyncBlocker,
    last: Option<LogPath>,
    added: Arc<Mutex<VecDeque<Result<LogFile, LogError>>>>,
    _watcher: RecommendedWatcher,
}

impl AsyncLiveDirIter {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<AsyncLiveDirIter, LogError> {
        let dir_iter = DirIter::new_async(path.as_ref()).await?;

        let blocker = AsyncBlocker::default();
        let added = Arc::new(Mutex::new(VecDeque::new()));

        let local_blocker = blocker.clone();
        let local_added = added.clone();

        // This is stopped when it is dropped
        let mut watcher = notify::recommended_watcher(move |event: notify::Result<Event>| {
            let Ok(event) = event else {
                return;
            };

            if !matches!(event.kind, EventKind::Create(CreateKind::File)) {
                return;
            }

            let Ok(mut lock) = local_added.lock() else {
                return;
            };

            for path in event.paths {
                let path = match LogPath::try_from(path.as_path()) {
                    Ok(path) => path,
                    Err(LogError::IncorrectFileName) => continue,
                    Err(error) => {
                        lock.push_back(Err(error));
                        continue;
                    }
                };

                let log_file = LogFile::from(path);

                lock.push_back(Ok(log_file));
            }

            local_blocker.unblock();
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(AsyncLiveDirIter {
            dir_iter,
            blocker,
            last: None,
            added,
            _watcher: watcher,
        })
    }

    pub fn is_last(&self, log_file: &LogFile) -> bool {
        let Some(last) = &self.last else {
            return false;
        };

        log_file == last
    }

    async fn inner_next(&mut self) -> Option<Result<LogFile, LogError>> {
        if let Some(entry) = self.dir_iter.next() {
            self.last = Some(entry.log_path().clone());
            // entry.set_blocker(Arc::new(self.blocker.clone()));

            return Some(Ok(entry));
        }

        loop {
            let added_value = self
                .added
                .lock()
                .expect("lock should have been acquired")
                .pop_front();

            if added_value.is_some() {
                return added_value;
            }

            self.blocker.block().await;
        }
    }
}

impl Stream for AsyncLiveDirIter {
    type Item = Result<LogFile, LogError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        pin!(self.inner_next()).poll_unpin(cx)
    }
}

#[cfg(test)]
mod tests {

    use crate::io::models::log_dir::async_live_dir_iter::AsyncLiveDirIter;
    use crate::modules::shared::asynchronous::async_blocker::AsyncBlocker;
    use crate::tests::test_dir;
    use futures::StreamExt;
    use smol::{fs, spawn};

    // #[ignore]
    fn async_live_watcher_blocks_correctly() {
        smol::block_on(async {
            let dir = test_dir();
            let first_file = dir.path().join("Journal.2023-02-21T084116.01.log");
            let second_file = dir.path().join("Journal.2023-02-21T084116.02.log");

            fs::write(
                &first_file,
                r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#,
            )
                .await
                .unwrap();

            let blocker = AsyncBlocker::new();

            let local_blocker = blocker.clone();
            let local_path = dir.path();

            let handle1 = spawn(async move {
                let mut live_dir = AsyncLiveDirIter::new(local_path).await.unwrap();
                let mut file = live_dir.next().await.unwrap().unwrap().live_iter().unwrap();

                assert!(file.next().is_some());

                local_blocker.unblock();
                dbg!();
                assert!(file.next().is_none());
                dbg!();

                let mut next_file = live_dir.next().await.unwrap().unwrap().iter().unwrap();

                assert!(next_file.next().is_some());
                assert!(next_file.next().is_none());

                assert!(true);
            });

            let handle2 = spawn(async move {
                dbg!("Handle 2 start");
                blocker.block().await;
                dbg!("Unblocked");

                fs::write(
                    &second_file,
                    r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#,
                )
                    .await
                    .unwrap();

                dbg!("Handle 2 done");
            });

            handle1.await;
            handle2.await;
        });
    }
}
