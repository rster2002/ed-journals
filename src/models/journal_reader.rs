use std::fs::File;
use std::{io, mem};
use std::collections::VecDeque;
use std::io::{LineWriter, Read, Seek, SeekFrom, Write};
use std::string::FromUtf8Error;
use thiserror::Error;
use crate::models::journal_event::JournalEntry;

#[derive(Debug, Error)]
pub enum JournalReaderError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    Utf8Error(#[from] FromUtf8Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct JournalReader {
    file: File,
    position: usize,
    file_read_buffer: Vec<u8>,
    entry_buffer: VecDeque<JournalEntry>,
}

impl JournalReader {
    fn read_next(&mut self) -> Result<(), JournalReaderError> {
        {
            let mut line_writer = LineWriter::new(&mut self.file_read_buffer);

            let mut read_buffer = Vec::new();
            self.file.seek(SeekFrom::Start(self.position as u64))?;
            self.position += self.file.read_to_end(&mut read_buffer)?;

            line_writer.write_all(&read_buffer)?;
        }

        let local_string = String::from_utf8(mem::take(&mut self.file_read_buffer))?;

        for line in local_string.lines() {
            dbg!(&line);
            self.entry_buffer.push_back(serde_json::from_str(line)?);
        }

        Ok(())
    }
}

impl From<File> for JournalReader  {
    fn from(value: File) -> Self {
        JournalReader  {
            file: value,
            position: 0,
            file_read_buffer: Vec::new(),
            entry_buffer: VecDeque::new(),
        }
    }
}

impl Iterator for JournalReader {
    type Item = Result<JournalEntry, JournalReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(err) = self.read_next() {
            return Some(Err(err));
        }

        self.entry_buffer.pop_front().map(|a| Ok(a))
    }
}
