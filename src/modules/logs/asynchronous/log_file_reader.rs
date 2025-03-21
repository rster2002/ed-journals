use std::collections::VecDeque;
use std::io;
use std::io::SeekFrom;
use std::path::Path;

use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncSeekExt;

use crate::logs::content::LogEvent;

#[derive(Debug)]
pub struct LogFileReader {
    source: File,
    position: usize,
    file_read_buffer: String,
    entry_buffer: VecDeque<Result<LogEvent, LogFileReaderError>>,
    failing: bool,
}

#[derive(Debug, Error)]
pub enum LogFileReaderError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error("Failed to parse log line: {0}")]
    FailedToParseLine(#[from] serde_json::Error),
}

impl LogFileReader {
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self, LogFileReaderError> {
        Ok(LogFileReader {
            source: File::open(path).await?,
            position: 0,
            file_read_buffer: String::new(),
            entry_buffer: VecDeque::new(),
            failing: false,
        })
    }

    async fn read_next(&mut self) -> Result<(), LogFileReaderError> {
        self.source
            .seek(SeekFrom::Start(self.position as u64))
            .await?;
        self.position += self
            .source
            .read_to_string(&mut self.file_read_buffer)
            .await?;

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
        self.file_read_buffer = String::new();

        Ok(())
    }

    pub async fn next(&mut self) -> Option<Result<LogEvent, LogFileReaderError>> {
        if self.failing {
            return None;
        }

        let result = self.read_next().await;

        if let Err(error) = result {
            self.failing = true;
            return Some(Err(error));
        }

        self.entry_buffer.pop_front()
    }
}