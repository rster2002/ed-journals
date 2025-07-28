use async_fs::File;
use futures::io::BufReader;
use futures::{FutureExt, Stream, StreamExt};
use notify::event::{DataChange, ModifyKind, RemoveKind};
use notify::{Event, EventKind, RecommendedWatcher, Watcher};
use std::path::Path;
use std::pin::{pin, Pin};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
// use tokio::fs::File;
// use tokio::io::BufReader;
// use tokio_util::compat::Compat;
use crate::io::{AsyncIter, LogError};
use crate::logs::LogEvent;
use crate::modules::shared::asynchronous::async_blocker::AsyncBlocker;

pub struct LiveAsyncIter {
    inner: AsyncIter<BufReader<File>>,
    blocker: Arc<AsyncBlocker>,
    removed: Arc<AtomicBool>,
    _watcher: RecommendedWatcher,
}

impl LiveAsyncIter {
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<LiveAsyncIter, LogError> {
        LiveAsyncIter::open_with_blocker(path, Arc::new(AsyncBlocker::default())).await
    }

    pub async fn open_with_blocker<P: AsRef<Path>>(
        path: P,
        blocker: Arc<AsyncBlocker>,
    ) -> Result<LiveAsyncIter, LogError> {
        let file = File::open(path.as_ref()).await?;
        let buf_reader = BufReader::new(file);

        let removed = Arc::new(AtomicBool::new(false));
        let inner = AsyncIter::from(buf_reader);

        let local_path = path.as_ref().to_owned();
        let local_removed = removed.clone();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |event: notify::Result<Event>| {
            let Ok(event) = event else {
                return;
            };

            if !local_path.exists() {
                local_removed.store(true, Ordering::Relaxed);
                local_blocker.unblock();
                return;
            }

            match event.kind {
                EventKind::Modify(ModifyKind::Any)
                | EventKind::Modify(ModifyKind::Data(DataChange::Any)) => {
                    local_blocker.unblock();
                }
                EventKind::Remove(RemoveKind::Any) => {
                    local_removed.store(true, Ordering::Relaxed);
                    local_blocker.unblock();
                }
                _ => {}
            }
        })?;

        watcher.watch(path.as_ref(), notify::RecursiveMode::NonRecursive)?;

        Ok(LiveAsyncIter {
            inner,
            removed,
            blocker,
            _watcher: watcher,
        })
    }

    async fn inner_next(&mut self) -> Option<Result<LogEvent, LogError>> {
        if let Some(event) = self.inner.next().await {
            return Some(event);
        }

        self.blocker.block().await;

        if self.removed.load(Ordering::Relaxed) {
            return None;
        }

        self.inner.next().await
    }
}

impl Stream for LiveAsyncIter {
    type Item = Result<LogEvent, LogError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        pin!(self.inner_next()).poll_unpin(cx)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use futures::StreamExt;
    use crate::io::LiveAsyncIter;
    use crate::modules::tests::simulate_log_file;

    #[test]
    fn it_works() {
        let path = simulate_log_file("live_watcher_blocks_correctly");

        smol::block_on(async {
            let mut live_async_reader = LiveAsyncIter::open(&path)
                .await
                .unwrap();

            let mut i = 0;
            let mut instant = Instant::now();
            while let Some(entry) = live_async_reader.next().await {
                i += 1;
                assert!(entry.is_ok());

                // Simulation sleeps for 100 ms, so if ~100 ms have passed, we can be sure that the
                // blocking has worked.
                assert!(instant.elapsed().as_millis() > 90);

                instant = Instant::now();

                if i > 20 {
                    return;
                }
            }

            unreachable!();
        });
    }
}
