use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use super::LogFileReaderError;

/// Used for reading entries from a single journal log file. Unlike the [log_file_reader::LogFileReader], this iterator does *not* parse the individual items. Instead you are getting [serde_json::Value]s.
/// [log_file_reader::LogFileReader] uses this reader under the hood.
#[derive(Debug)]
pub struct RawLogFileReader {
    source: File,
    position: usize,
    file_read_buffer: String,
    entry_buffer: VecDeque<Result<serde_json::Value, LogFileReaderError>>,
    failing: bool,
}

impl RawLogFileReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, LogFileReaderError> {
        Ok(RawLogFileReader {
            source: File::open(path)?,
            position: 0,
            file_read_buffer: String::new(),
            entry_buffer: VecDeque::new(),
            failing: false,
        })
    }

    fn read_next(&mut self) -> Result<(), LogFileReaderError> {
        self.source.seek(SeekFrom::Start(self.position as u64))?;
        self.position += self.source.read_to_string(&mut self.file_read_buffer)?;

        // Set position back one space to ensure the reader doesn't skip a character during the
        // next read.
        if self.file_read_buffer.ends_with('\n') {
            self.position -= 1;
        }

        let mut lines = self
            .file_read_buffer
            .lines()
            .filter(|line| !line.trim().is_empty())
            .peekable();

        while let Some(line) = lines.next() {
            let parse_result = serde_json::from_str(line.trim_matches('\0'));

            #[cfg(test)]
            if parse_result.is_err() {
                dbg!(&line);
                dbg!(&parse_result);
            }

            // If the line didn't parse, but the line is the last line that was read, it will not
            // error and instead add the current line back into the read buffer to hopefully be
            // completed when new lines are added.
            if parse_result.is_err() && lines.peek().is_none() {
                self.file_read_buffer = line.to_string();
                return Ok(());
            }

            self.entry_buffer
                .push_back(parse_result.map_err(|e| e.into()));
        }

        // If it reaches this point that means that the whole read buffer has been processed, so it
        // can be cleared.
        self.file_read_buffer.clear();

        Ok(())
    }
}

impl Iterator for RawLogFileReader {
    type Item = Result<serde_json::Value, LogFileReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        // If the reader has failed it will not return any new lines.
        if self.failing {
            return None;
        }

        let result = self.read_next();

        // If an error has been returned at this location that means that it is something that
        // cannot be recovered from.
        if let Err(error) = result {
            self.failing = true;
            return Some(Err(error));
        }

        self.entry_buffer.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use crate::logs::blocking::raw_log_file_reader::RawLogFileReader;

    #[test]
    fn partial_last_lines_are_read_correctly() {
        fs::write("a.tmp", "").unwrap();
        let mut reader = RawLogFileReader::open("a.tmp").unwrap();

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
            reader
                .next()
                .unwrap()
                .unwrap()
                .as_object()
                .unwrap()
                .get("event")
                .unwrap()
                .as_str()
                .unwrap(),
            "Fileheader"
        );
        assert_eq!(
            reader
                .next()
                .unwrap()
                .unwrap()
                .as_object()
                .unwrap()
                .get("event")
                .unwrap()
                .as_str()
                .unwrap(),
            "Commander"
        );

        assert!(reader.next().is_none());

        fs::remove_file("a.tmp").unwrap();
    }

    #[test]
    fn partial_last_lines_are_read_correctly_2() {
        fs::write("d.tmp", "").unwrap();
        let mut reader = RawLogFileReader::open("d.tmp").unwrap();

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
            reader
                .next()
                .unwrap()
                .unwrap()
                .as_object()
                .unwrap()
                .get("event")
                .unwrap()
                .as_str()
                .unwrap(),
            "Fileheader"
        );

        assert!(reader.next().is_none());

        fs::remove_file("d.tmp").unwrap();
    }

    #[test]
    fn partial_last_lines_are_read_correctly_3() {
        fs::write("e.tmp", "").unwrap();
        let mut reader = RawLogFileReader::open("e.tmp").unwrap();

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
            reader.file_read_buffer,
            r#"{"timestamp":"2022-10-22T15:12:05Z","#
        );

        fs::write("e.tmp", r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}
{"timestamp":"2022-10-22T15:12:05Z","event":"Commander","FID":"F123456789","Name":"TEST"}
"#)
            .unwrap();

        assert_eq!(
            reader
                .next()
                .unwrap()
                .unwrap()
                .as_object()
                .unwrap()
                .get("event")
                .unwrap()
                .as_str()
                .unwrap(),
            "Commander"
        );

        assert!(reader.next().is_none());

        fs::remove_file("e.tmp").unwrap();
    }

    #[test]
    fn incorrect_lines_return_an_err_only_when_it_is_expected() {
        fs::write("b.tmp", "").unwrap();
        let mut reader = RawLogFileReader::open("b.tmp").unwrap();

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
            reader
                .next()
                .unwrap()
                .unwrap()
                .as_object()
                .unwrap()
                .get("event")
                .unwrap()
                .as_str()
                .unwrap(),
            "Fileheader"
        );

        // The second line should return an error. The above example it unlikely to happen and this
        // is most likely to happen because of some unknown format.
        assert!(reader.next().unwrap().is_err());

        // The next like should return like normal.
        assert_eq!(
            reader.next().unwrap().unwrap(),
            json!({"timestamp":"2022-10-22T15:12:33Z","event":"FSSSignalDiscovered","SystemAddress":5031654888146i64,"SignalName":"HMS CHUCKLE PHUCK J6K-8XT","IsStation":true}),
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
            reader.next().unwrap().unwrap(),
            json!({"timestamp":"2022-10-22T15:12:33Z","event":"FSSSignalDiscovered","SystemAddress":5031654888146i64,"SignalName":"BREAK OF DAWN V3T-G0Y","IsStation":true}),
        );

        fs::remove_file("b.tmp").unwrap();
    }

    #[test]
    fn last_lines_are_read_correctly() {
        fs::write("c.tmp", "").unwrap();
        let mut reader = RawLogFileReader::open("c.tmp").unwrap();

        assert!(reader.next().is_none());

        fs::write(
            "c.tmp",
            r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}"#,
        )
            .unwrap();

        assert_eq!(
            reader
                .next()
                .unwrap()
                .unwrap()
                .as_object()
                .unwrap()
                .get("event")
                .unwrap()
                .as_str()
                .unwrap(),
            "Fileheader"
        );

        fs::write("c.tmp", r#"{"timestamp":"2022-10-22T15:10:41Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.0.0.1450","build":"r286858/r0 "}
{"timestamp":"2022-10-22T15:12:05Z","event":"Commander","FID":"F123456789","Name":"TEST"}"#)
            .unwrap();

        assert_eq!(
            reader
                .next()
                .unwrap()
                .unwrap()
                .as_object()
                .unwrap()
                .get("event")
                .unwrap()
                .as_str()
                .unwrap(),
            "Commander"
        );

        fs::remove_file("c.tmp").unwrap();
    }
}
