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

mod modules;

pub use modules::logs;
pub use modules::journal;
pub use modules::state;

pub use modules::status;
pub use modules::outfitting;
pub use modules::shipyard;
pub use modules::market;
pub use modules::nav_route;
pub use modules::modules_info;
pub use modules::backpack;
pub use modules::ship_locker;

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::logs::LogDir;
    use crate::logs::content::LogEventContent;

    #[test]
    fn test_journals_are_parsed_correctly() {
        let dir_path = current_dir()
            .unwrap()
            .join("test-files")
            .join("journals");

        let log_dir = LogDir::new(dir_path);

        let logs = log_dir.journal_logs().unwrap();

        assert!(logs.len() > 10);

        let mut file_header_count = 0;
        let mut entry_count = 0;

        for journal in &logs {
            let mut found_file_header = false;
            let reader = journal.create_blocking_reader().unwrap();

            for entry in reader {
                entry_count += 1;

                if let LogEventContent::FileHeader(_) = entry.unwrap().content {
                    found_file_header = true;
                    file_header_count += 1;
                }
            }
        }

        dbg!(file_header_count);
        dbg!(entry_count);

        // assert_eq!(logs.len(), file_header_count);
    }
}
