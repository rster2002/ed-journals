use std::io::{Bytes, Read};
use crate::logs::LogEvent;
use crate::modules::logs2::error::LogError;

/// Standard iterator for iterating over some [Read] and returning [LogEvents](LogEvent).
pub struct LogIter<T>
where T : Read
{
    inner: Bytes<T>,
}

impl<T> From<T> for LogIter<T>
where T : Read
{
    fn from(value: T) -> Self {
        LogIter {
            inner: value.bytes(),
        }
    }
}

impl<T> Iterator for LogIter<T>
where T : Read
{
    type Item = Result<LogEvent, LogError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = Vec::with_capacity(64); // Line are mostly at least 64 bytes

        while let Some(byte) = self.inner.next() {
            let byte = match byte {
                Ok(byte) => byte,
                Err(e) => return Some(Err(e.into())),
            };

            if byte == b'\n' && !line.is_empty() {
                break;
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{BufReader, Cursor};
    use std::time::Instant;
    use crate::logs::blocking::LogFileReader;
    use crate::logs::LogEventContentKind;
    use crate::modules::logs2::models::log_iter::LogIter;

    #[test]
    fn log_reader_reads_completed_file_correctly() {
        let data = r#"{ "timestamp":"2020-09-21T19:04:44Z", "event":"Repair", "Item":"Paint", "Cost":1 }
{ "timestamp":"2020-09-21T19:04:51Z", "event":"Repair", "Item":"Wear", "Cost":10 }"#;

        let cursor = Cursor::new(data);
        let mut reader = LogIter::from(cursor);

        assert!(reader.next().is_some());
        assert!(reader.next().is_some());

        assert!(dbg!(reader.next()).is_none());
    }

    #[test]
    fn log_reader_handles_trailing_newlines_correctly() {
        let data = r#"{ "timestamp":"2020-09-21T19:04:44Z", "event":"Repair", "Item":"Paint", "Cost":1 }
{ "timestamp":"2020-09-21T19:04:51Z", "event":"Repair", "Item":"Wear", "Cost":10 }
"#;

        let cursor = Cursor::new(data);
        let mut reader = LogIter::from(cursor);

        assert!(reader.next().is_some());
        assert!(reader.next().is_some());

        assert!(dbg!(reader.next()).is_none());
    }

    #[test]
    fn last_lines_are_read_correctly() {
        fs::write("c.tmp", "").unwrap();
        let file = File::open("c.tmp").unwrap();
        let buf_reader = BufReader::new(file);

        let mut reader = LogIter::from(buf_reader);

        assert!(reader.next().is_none());

        fs::write(
            "c.tmp",
            r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#,
        )
            .unwrap();

        assert_eq!(
            reader.next().unwrap().unwrap().content.kind(),
            LogEventContentKind::FileHeader
        );

        fs::write("c.tmp", r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}
{"timestamp":"2022-10-22T15:12:05Z","event":"Commander","FID":"F123456789","Name":"TEST"}"#)
            .unwrap();

        assert_eq!(
            reader.next().unwrap().unwrap().content.kind(),
            LogEventContentKind::Commander
        );

        fs::remove_file("c.tmp").unwrap();
    }
}