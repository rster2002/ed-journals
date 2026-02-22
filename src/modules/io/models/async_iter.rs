use crate::logs::LogEvent;
use crate::modules::io::error::LogIOError;
use futures::{AsyncRead, AsyncReadExt, FutureExt, Stream};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::pin::{pin, Pin};
use std::task::{Context, Poll};

/// Asynchronous iterator for iterating over some [AsyncRead] and returning [serde_json::Value]s
/// which can then manually be parsed using [serde_json::from_value]. To automatically parse
/// entries, use [AsyncIter](crate::io::AsyncIter) instead.
pub struct AsyncIter<T, R = LogEvent>
where
    T: AsyncRead + Unpin,
    R: DeserializeOwned + Unpin,
{
    inner: T,
    _p: PhantomData<R>,
}

impl<T> AsyncIter<T>
where
    T: AsyncRead + Unpin,
{
    pub fn new(inner: T) -> AsyncIter<T, LogEvent> {
        AsyncIter {
            inner,
            _p: PhantomData,
        }
    }

    pub fn new_raw(inner: T) -> AsyncIter<T, serde_json::Value> {
        AsyncIter {
            inner,
            _p: PhantomData,
        }
    }
}

impl<T, R> AsyncIter<T, R>
where
    T: AsyncRead + Unpin,
    R: DeserializeOwned + Unpin,
{
    async fn inner_next(&mut self) -> Option<Result<R, LogIOError>> {
        let mut line = Vec::with_capacity(64);

        loop {
            let mut buf: [u8; 1] = [0; 1];
            let result = self.inner.read(&mut buf).await;

            match result {
                Ok(0) => break,
                Ok(_) => {}
                Err(e) => return Some(Err(e.into())),
            }

            let byte = buf[0];

            if byte == b'\n' && !line.is_empty() {
                break;
            }

            if byte == 0x00 || (line.is_empty() && byte == b' ') {
                continue;
            }

            line.push(byte);
        }

        if line.is_empty() {
            return None;
        }

        Some(Ok(match serde_json::from_slice(&line) {
            Ok(event) => event,
            Err(e) => return Some(Err(e.into())),
        }))
    }
}

impl<T, R> From<T> for AsyncIter<T, R>
where
    T: AsyncRead + Unpin,
    R: DeserializeOwned + Unpin,
{
    fn from(inner: T) -> Self {
        AsyncIter {
            inner,
            _p: PhantomData,
        }
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl<A> From<A> for AsyncIter<tokio_util::compat::Compat<A>>
where
    A: tokio::io::AsyncRead + Unpin,
{
    fn from(value: A) -> Self {
        let compat = tokio_util::compat::TokioAsyncReadCompatExt::compat(value);
        AsyncIter::from(compat)
    }
}

impl<T, R> Stream for AsyncIter<T, R>
where
    T: AsyncRead + Unpin,
    R: DeserializeOwned + Unpin,
{
    type Item = Result<R, LogIOError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        pin!(self.inner_next()).poll_unpin(cx)
    }
}

#[cfg(test)]
mod tests {
    use crate::logs::LogEventContentKind;
    use crate::modules::io::AsyncIter;
    use async_fs::File;
    use futures::io::Cursor;
    use futures::StreamExt;
    use smol::fs;

    fn async_reader_reads_complete_file_correctly() {
        smol::block_on(async {
            let data = r#"{ "timestamp":"2020-09-21T19:04:44Z", "event":"Repair", "Item":"Paint", "Cost":1 }
{ "timestamp":"2020-09-21T19:04:51Z", "event":"Repair", "Item":"Wear", "Cost":10 }"#;

            let cursor = Cursor::new(data);
            let buf_reader = futures::io::BufReader::new(cursor);

            let mut reader = AsyncIter::new(buf_reader);

            assert!(reader.next().await.is_some());
            assert!(reader.next().await.is_some());

            assert!(dbg!(reader.next().await).is_none());
        });
    }

    fn last_lines_are_read_correctly() {
        smol::block_on(async {
            fs::write("c.tmp", "").await.unwrap();

            let file = File::open("c.tmp").await.unwrap();

            let buf_reader = futures::io::BufReader::new(file);

            let mut reader = AsyncIter::new(buf_reader);

            assert!(reader.next().await.is_none());

            fs::write(
                "c.tmp",
                r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#,
            )
                .await
                .unwrap();

            assert_eq!(
                reader.next().await.unwrap().unwrap().content.kind(),
                LogEventContentKind::FileHeader
            );

            fs::write("c.tmp", r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}
{"timestamp":"2022-10-22T15:12:05Z","event":"Commander","FID":"F123456789","Name":"TEST"}"#)
                .await
                .unwrap();

            assert_eq!(
                reader.next().await.unwrap().unwrap().content.kind(),
                LogEventContentKind::Commander
            );

            fs::remove_file("c.tmp").await.unwrap();
        });
    }
}
