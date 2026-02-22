use crate::logs::LogEvent;
use crate::modules::io::error::LogIOError;
use std::io::Read;

/// Standard iterator for iterating over some [Read] and returning raw [serde_json::Value]s. You can
/// read the contents of a file by wrapping a file with this iterator. You can then manually parse
/// entries to [LogEvent]s by calling [serde_json::from_value]. To automatically parse events you
/// can use [LogIter](crate::io::LogIter) instead.
///
/// ```rust
/// use std::fs::File;
/// use std::io::BufReader;
/// use ed_journals::io::RawIter;
///
/// let file = File::open("./test-files/journals/Journal.2022-10-11T214552.01.log")
///     .unwrap();
/// let buf_reader = BufReader::new(file);
///
/// let mut iterator = RawIter::from(buf_reader);
///
/// assert!(iterator.next().is_some());
/// ```
#[derive(Debug)]
pub struct RawIter<T>
where
    T: Read,
{
    inner: T,
}

impl<T> From<T> for RawIter<T>
where
    T: Read,
{
    fn from(value: T) -> Self {
        RawIter { inner: value }
    }
}

impl<T> Iterator for RawIter<T>
where
    T: Read,
{
    type Item = Result<serde_json::Value, LogIOError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = Vec::with_capacity(64); // Line are mostly at least 64 bytes

        let mut buf = [0u8; 1];
        loop {
            let size = match self.inner.read(&mut buf) {
                Ok(size) => size,
                Err(e) => return Some(Err(e.into())),
            };

            if size == 0 {
                break;
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
            Err(e) => {
                #[cfg(test)]
                dbg!(&line);

                return Some(Err(e.into()));
            }
        }))
    }
}
