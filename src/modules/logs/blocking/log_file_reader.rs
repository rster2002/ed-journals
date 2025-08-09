use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

use thiserror::Error;

use crate::logs::content::LogEvent;

use super::RawLogFileReader;

/// Used for reading entries from a single journal log file. The reader takes care of things like
/// partial lines if that ever happens and parsing them to a usable [JournalEvent].
///
/// You'll almost always create a reader by calling the [JournalFile::create_reader] function on a
/// [JournalFile]:
///
/// ```rust
/// use std::env::current_dir;
/// use ed_journals::logs::LogDir;
///
/// let dir_path = current_dir()
///     .unwrap()
///     .join("test-files")
///     .join("journals");
///
/// let journal_dir = LogDir::new(dir_path);
///
/// let logs = journal_dir.journal_logs_oldest_first().unwrap();
/// assert!(!logs.is_empty());
///
/// for journal_file in logs {
///     // Create a reader
///     let reader = journal_file.create_blocking_reader().unwrap();
///
///     for entry in reader {
///         let Ok(log) = entry else {
///             println!("Unreadable line");
///             continue;
///         };
///
///         // Do something with the entry
///     }
/// }
/// ```
#[derive(Debug)]
pub struct LogFileReader {
    inner: RawLogFileReader,
}

#[derive(Debug, Error)]
pub enum LogFileReaderError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    Utf8Error(#[from] FromUtf8Error),

    #[error("Failed to parse log line: {0}")]
    FailedToParseLine(#[from] serde_json::Error),
}

impl LogFileReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, LogFileReaderError> {
        Ok(LogFileReader {
            inner: RawLogFileReader::open(path)?,
        })
    }
}

impl Iterator for LogFileReader {
    type Item = Result<LogEvent, LogFileReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.inner.next()? {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };

        Some(serde_json::from_value(result).map_err(|e| e.into()))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::logs::blocking::LogFileReader;
    use crate::logs::content::log_event_content::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
    use crate::logs::content::log_event_content::LogEventContentKind;
    use crate::logs::content::LogEventContent;

    #[test]
    fn partial_last_lines_are_read_correctly() {
        fs::write("a.tmp", "").unwrap();
        let mut reader = LogFileReader::open("a.tmp").unwrap();

        assert!(reader.next().is_none());

        fs::write(
            "a.tmp",
            r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"#,
        )
        .unwrap();

        assert!(reader.next().is_none());

        fs::write("a.tmp", r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}
{"timestamp":"2022-10-22T15:12:05Z","event":"Commander","FID":"F123456789","Name":"TEST"}"#)
            .unwrap();

        assert_eq!(
            reader.next().unwrap().unwrap().content.kind(),
            LogEventContentKind::FileHeader
        );
        assert_eq!(
            reader.next().unwrap().unwrap().content.kind(),
            LogEventContentKind::Commander
        );

        assert!(reader.next().is_none());

        fs::remove_file("a.tmp").unwrap();
    }

    #[test]
    fn partial_last_lines_are_read_correctly_2() {
        fs::write("d.tmp", "").unwrap();
        let mut reader = LogFileReader::open("d.tmp").unwrap();

        assert!(reader.next().is_none());

        fs::write(
            "d.tmp",
            r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"#,
        )
        .unwrap();

        assert!(reader.next().is_none());

        fs::write("d.tmp", r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","#)
            .unwrap();

        assert!(reader.next().is_none());

        fs::write("d.tmp", r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#)
            .unwrap();

        assert_eq!(
            reader.next().unwrap().unwrap().content.kind(),
            LogEventContentKind::FileHeader
        );

        assert!(reader.next().is_none());

        fs::remove_file("d.tmp").unwrap();
    }

    #[test]
    fn partial_last_lines_are_read_correctly_3() {
        fs::write("e.tmp", "").unwrap();
        let mut reader = LogFileReader::open("e.tmp").unwrap();

        assert!(reader.next().is_none());

        fs::write(
            "e.tmp",
            r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#,
        )
            .unwrap();

        assert!(reader.next().is_some());

        fs::write("e.tmp", r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}
{"timestamp":"2022-10-22T15:12:05Z",
"#)
            .unwrap();

        assert!(reader.next().is_none());
        assert_eq!(
            reader.inner.file_read_buffer,
            r#"{"timestamp":"2022-10-22T15:12:05Z","#
        );

        fs::write("e.tmp", r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}
{"timestamp":"2022-10-22T15:12:05Z","event":"Commander","FID":"F123456789","Name":"TEST"}
"#)
            .unwrap();

        assert_eq!(
            reader.next().unwrap().unwrap().content.kind(),
            LogEventContentKind::Commander
        );

        assert!(reader.next().is_none());

        fs::remove_file("e.tmp").unwrap();
    }

    #[test]
    fn incorrect_lines_return_an_err_only_when_it_is_expected() {
        fs::write("b.tmp", "").unwrap();
        let mut reader = LogFileReader::open("b.tmp").unwrap();

        assert!(reader.next().is_none());

        fs::write(
            "b.tmp",
            r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}
{"timestamp":"2022-10-22T15:12:05Z","event":"Commander","FID":"F123456789","Na BADLY FORMATTED
{"timestamp":"2022-10-22T15:12:33Z","event":"FSSSignalDiscovered","SystemAddress":5031654888146,"SignalName":"HMS CHUCKLE PHUCK J6K-8XT","IsStation":true}
{"timestamp":"2022-10-22T15:12:33Z","event":"FSSSignalDiscovered","SystemAddress":5031654888146,"#,
        )
            .unwrap();

        // The first like should parse like expected
        assert_eq!(
            reader.next().unwrap().unwrap().content.kind(),
            LogEventContentKind::FileHeader
        );

        // The second line should return an error. The above example it unlikely to happen and this
        // is most likely to happen because of some unknown format.
        assert!(reader.next().unwrap().is_err());

        // The next like should return like normal.
        assert_eq!(
            reader.next().unwrap().unwrap().content,
            LogEventContent::FSSSignalDiscovered(FSSSignalDiscoveredEvent {
                system_address: 5031654888146,
                signal_name: "HMS CHUCKLE PHUCK J6K-8XT".to_string(),
                signal_name_localized: None,
                is_station: true,
            }),
        );

        // The last line, even though it's not correctly formatted should return None as it's the
        // last line and could just be impartial.
        assert!(reader.next().is_none());

        fs::write(
            "b.tmp",
            r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}
{"timestamp":"2022-10-22T15:12:05Z","event":"Commander","FID":"F123456789","Na BADLY FORMATTED
{"timestamp":"2022-10-22T15:12:33Z","event":"FSSSignalDiscovered","SystemAddress":5031654888146,"SignalName":"HMS CHUCKLE PHUCK J6K-8XT","IsStation":true}
{"timestamp":"2022-10-22T15:12:33Z","event":"FSSSignalDiscovered","SystemAddress":5031654888146,"SignalName":"BREAK OF DAWN V3T-G0Y","IsStation":true}"#,
        )
            .unwrap();

        // The when the last line is completed, the line should be parsed and returned correctly.
        assert_eq!(
            reader.next().unwrap().unwrap().content,
            LogEventContent::FSSSignalDiscovered(FSSSignalDiscoveredEvent {
                system_address: 5031654888146,
                signal_name: "BREAK OF DAWN V3T-G0Y".to_string(),
                signal_name_localized: None,
                is_station: true,
            }),
        );

        fs::remove_file("b.tmp").unwrap();
    }

    #[test]
    fn last_lines_are_read_correctly() {
        fs::write("c.tmp", "").unwrap();
        let mut reader = LogFileReader::open("c.tmp").unwrap();

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
