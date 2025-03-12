use std::pin::Pin;
use std::task::{Context, Poll};
use futures::{FutureExt, Stream};
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::logs::LogEvent;

pub struct AsyncReader<T>
where T : AsyncRead + Unpin
{
    inner: T,
}

impl<T> AsyncReader<T>
where T : AsyncRead + Unpin
{
    pub fn new(inner: T) -> AsyncReader<T> {
        AsyncReader {
            inner,
        }
    }

    async fn test(&self) -> Option<LogEvent> {
        todo!()
    }
}

impl<T> Stream for AsyncReader<T>
where T : AsyncRead + Unpin
{
    type Item = LogEvent;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        match this.test().poll_unpin(cx) {
            Poll::Ready(result) => Poll::Ready(result),
            Poll::Pending => Poll::Pending,
        }

        // match self.inner.poll_read(cx) {
        //     Poll::Ready(data) => {
        //         self.inner.read()
        //     }
        //     Poll::Pending => Poll::Pending,
        // }
    }
}