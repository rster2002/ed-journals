use std::{io, mem};
use std::collections::VecDeque;
use std::io::{Read, Seek, SeekFrom};
use std::string::FromUtf8Error;

use thiserror::Error;

use crate::models::journal_event::JournalEvent;

/// Used for reading entries from a journal log file. The reader takes care of things like partial
/// lines if that ever happens and parsing them to a usable [JournalEvent].
///
/// You'll almost always create a reader by calling the [JournalFile::create_reader] function on a
/// [JournalFile]:
///
/// ```rust
/// use std::env::current_dir;
/// use ed_journals::{JournalDir, JournalFile};
///
/// let dir_path = current_dir().unwrap()
///     .parent()
///     .unwrap()
///     .join("test-journals");
///
/// let journal_dir = JournalDir::try_from(dir_path)
///     .unwrap();
///
/// let logs = journal_dir.journal_logs().unwrap();
/// assert!(!logs.is_empty());
///
/// for journal_file in logs {
///     // Create a reader
///     let reader = journal_file.create_reader().unwrap();
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
pub struct JournalReader<T>
where
    T: Read + Seek,
{
    source: T,
    position: usize,
    file_read_buffer: String,
    entry_buffer: VecDeque<JournalEvent>,
}

#[derive(Debug, Error)]
pub enum JournalReaderError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    Utf8Error(#[from] FromUtf8Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

impl<T> JournalReader<T>
where
    T: Read + Seek,
{
    fn read_next(&mut self) -> Result<(), JournalReaderError> {
        self.source.seek(SeekFrom::Start(self.position as u64))?;
        self.position += self.source.read_to_string(&mut self.file_read_buffer)?;

        let mut lines = self.file_read_buffer.lines().peekable();

        loop {
            let Some(line) = lines.next() else {
                break;
            };

            if lines.peek().is_none() {
                if self.file_read_buffer == line {
                    break;
                }

                // Try to parse the remainder. If it does parse, it also counts processed.
                if let Ok(entry) = serde_json::from_str(line) {
                    self.entry_buffer.push_back(entry);
                    self.file_read_buffer = String::new();
                    break;
                }

                let new_line = line.to_string();
                let _ = mem::replace(&mut self.file_read_buffer, new_line);
                break;
            }

            let result = serde_json::from_str(line);

            if result.is_err() {
                dbg!(&line);
            }

            self.entry_buffer.push_back(result?);
        }

        Ok(())
    }
}

impl<T> From<T> for JournalReader<T>
where
    T: Read + Seek,
{
    fn from(value: T) -> Self {
        JournalReader {
            source: value,
            position: 0,
            file_read_buffer: String::new(),
            entry_buffer: VecDeque::new(),
        }
    }
}

impl<T> Iterator for JournalReader<T>
where
    T: Read + Seek,
{
    type Item = Result<JournalEvent, JournalReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(err) = self.read_next() {
            return Some(Err(err));
        }

        self.entry_buffer.pop_front().map(Ok)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Cursor;

    use chrono::{TimeZone, Utc};

    use crate::models::journal_event::JournalEvent;
    use crate::models::journal_event_content::{JournalEventContent, JournalEventContentKind};
    use crate::models::journal_event_content::commander_event::CommanderEvent;
    use crate::models::journal_event_content::file_header_event::FileHeaderEvent;
    use crate::models::journal_reader::JournalReader;

    #[test]
    fn journal_events_are_read_in_the_correct_order() {
        let test_journal_contents =
            include_str!("../../test-files/Journal.2022-10-22T171117.01.log");
        let cursor = Cursor::new(test_journal_contents);

        let mut reader = JournalReader::from(cursor);

        let result = reader
            .next()
            .expect("Should be filled")
            .expect("Should not be an error");

        assert_eq!(
            result,
            JournalEvent {
                timestamp: Utc.with_ymd_and_hms(2022, 10, 22, 15, 10, 41).unwrap(), // 2022-10-22T15:10:41Z
                content: JournalEventContent::FileHeader(FileHeaderEvent {
                    part: 1,
                    language: "English/UK".to_string(),
                    odyssey: true,
                    game_version: "4.0.0.1450".to_string(),
                    build: "r286858/r0 ".to_string(),
                }),
            }
        );

        let result = reader
            .next()
            .expect("Should be filled")
            .expect("Should not be an error");

        assert_eq!(
            result,
            JournalEvent {
                timestamp: Utc.with_ymd_and_hms(2022, 10, 22, 15, 12, 05).unwrap(), // 2022-10-22T15:12:05Z
                content: JournalEventContent::Commander(CommanderEvent {
                    fid: "F123456789".to_string(),
                    name: "TEST".to_string(),
                }),
            }
        );
    }

    #[test]
    fn partial_lines_are_read_correctly() {
        fs::write("a.tmp", "").unwrap();

        let file = File::open("a.tmp").unwrap();

        let mut reader = JournalReader::from(file);

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
            JournalEventContentKind::FileHeader
        );
        assert_eq!(
            reader.next().unwrap().unwrap().content.kind(),
            JournalEventContentKind::Commander
        );

        fs::remove_file("a.tmp").unwrap();
    }
}
