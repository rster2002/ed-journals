use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Stream;
use tokio::fs::File;
use tokio::io::BufReader;
use tokio_util::compat::Compat;
use crate::io::{AsyncIter, LogError};

pub struct LiveAsyncIter
{
    inner: AsyncIter<Compat<BufReader<File>>>,
}

impl LiveAsyncIter {
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<LiveAsyncIter, LogError> {
        let file = File::open(path).await?;
        let buf_reader = BufReader::new(file);

        let inner = AsyncIter::from(buf_reader);

        Ok(LiveAsyncIter {
            inner
        })
    }
}

impl Stream for LiveAsyncIter {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use tokio::fs;
    use crate::io::models::log_file::live_async_iter::LiveAsyncIter;
    use crate::tests::{test_dir, test_file};

    #[tokio::test]
    async fn works() {
        fs::write("test.tmp", "").await.unwrap();

        let live_async_iter = LiveAsyncIter::open("test.tmp").await.unwrap();
    }
}
