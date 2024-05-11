//! # ED Journals
//!
//! This library provides models and utilities to work with Elite Dangerous journal files.
//!
//! > **Warning** this project is currently in beta, which means that it is very much work in progress. Breaking
//! > changes are likely to happen.
//!
//! ## Getting started
//!
//! Currently, the only files that are parsed are the log files. Models for working `Market.json`
//! and `Status.json` etc will be added in the future. Best place to get started is the [JournalDir]
//! model.
pub use models::journal_event_content;
pub use models::journal_dir::JournalDir;
pub use models::journal_dir::JournalDirError;
pub use models::journal_file::JournalFile;
pub use models::journal_file::JournalFileError;

pub use models::journal_event::JournalEvent;
pub use models::journal_event_content::JournalEventContent;
pub use models::journal_event_content::JournalEventContentKind;

#[cfg(feature = "blocking")]
pub use models::blocking;

#[cfg(feature = "async")]
pub use models::r#async;

mod macros;
mod models;

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::blocking::JournalDir;

    use crate::models::journal_event_content::JournalEventContent;

    #[test]
    fn test_journals_are_parsed_correctly() {
        let dir_path = current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .join("test-journals");

        let log_dir = JournalDir::new(dir_path).unwrap();

        let logs = log_dir.journal_logs().unwrap();

        assert!(logs.len() > 10);

        let mut file_header_count = 0;
        let mut entry_count = 0;

        for journal in &logs {
            let reader = journal.create_blocking_reader().unwrap();

            for entry in reader {
                entry_count += 1;

                if let JournalEventContent::FileHeader(_) = entry.unwrap().content {
                    file_header_count += 1;
                }
            }
        }

        dbg!(file_header_count);
        dbg!(entry_count);

        assert_eq!(logs.len(), file_header_count);
    }
}
